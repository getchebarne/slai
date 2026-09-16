use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::monsters::byrd;
use crate::relics::RELICS_COMBAT_START_PRE_DRAW;
use crate::relics::RELICS_COMBAT_START_TOP;
use crate::relics::RELICS_TURN_START_POST_DRAW;
use crate::relics::iter_owned_relics;
use crate::relics::trigger_relic_counter;
use crate::types::CardColor;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

pub fn process_effect_turn_start(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_turn_start outside the Combat frame"
    );
    let Combat {
        id_monsters,
        id_card_draw,
        energy,
        id_card_nightmare,
        turn,
        ..
    } = &mut state.combat;
    let id_actor = id_target.expect("TurnStart requires id_target");

    // Clear effect buffer
    state.effect_buf.clear();

    // Get mutable references
    let entity = &mut state.entities[id_actor];
    let modifiers = &mut entity.modifiers;
    let vitals = &mut entity.vitals;

    // Poison: queue Poison Tick
    if has_modifier(modifiers, ModifierKind::Poison) {
        state.effect_buf.push(Effect {
            kind: EffectKind::PoisonTick,
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    // Blur: existing block skips turn-stat reset
    let mut new_block: u16 = 0;
    if has_modifier(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }

    // Calipers: retain block minus 15 instead of losing all; max with Blur, never additive
    if id_actor == state.id_character && has_relic(&state.id_relics, RelicName::Calipers) {
        new_block = new_block.max(vitals.block.saturating_sub(15));
    }

    // Barricade: block never expires
    if has_modifier(modifiers, ModifierKind::Barricade) {
        new_block = new_block.max(vitals.block);
    }

    // Next turn block (Dodge and Roll)
    if has_modifier(modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(modifiers, ModifierKind::NextTurnBlock);
    }

    // Set new block value (should be zero most of the time)
    state.effect_buf.push(Effect {
        kind: EffectKind::BlockSet { amount: new_block },
        id_source: None,
        target: Target::Direct(Some(id_actor)),
    });

    // Phantasmal: gains double damage
    if has_modifier(modifiers, ModifierKind::Phantasmal) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DoubleDamage,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    // Flight: stacks refresh to the spawn value at the owner's turn start
    if has_modifier(modifiers, ModifierKind::Flight) {
        modifiers.stacks[ModifierKind::Flight as usize] = byrd::flight_stacks(state.ascension);
    }

    // Character's turn start, in GameActionManager's order; turn 1 follows AbstractRoom's
    if id_actor == state.id_character {
        let first_turn = *turn == 0;
        *turn += 1;
        let modifiers = state.entities[id_actor].modifiers;

        // Owned Relics in acquisition order
        let mut id_relics: Vec<usize> = iter_owned_relics(&state.id_relics)
            .map(|(_, id)| id)
            .collect();
        id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);

        // Turn 1: addToTop combat-start Relics run ahead of everything, newest pickup first
        if first_turn {
            for &id_relic in id_relics.iter().rev() {
                let relic = &state.entities[id_relic];
                if RELICS_COMBAT_START_TOP.contains(&relic.relic_name) {
                    for &effect in relic.relic_effects_combat_start {
                        state.effect_buf.push(effect);
                    }
                }
            }
        }

        // Round end: duration Modifiers tick (Character's, then Monsters')
        if !first_turn {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierTick,
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierTick,
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }

        // Persistent turn counters (Happy Flower, Incense Burner), spanning combats
        for name in [RelicName::HappyFlower, RelicName::IncenseBurner] {
            if let Some(id) = trigger_relic_counter(name, &state.id_relics, &mut state.entities) {
                for &effect in state.entities[id].relic_effects_counter {
                    state.effect_buf.push(effect);
                }
            }
        }

        // Horn Cleat and Captain's Wheel: one-shot turn counters
        // TODO: add combat turn # field to `GameState`
        for name in [RelicName::HornCleat, RelicName::CaptainsWheel] {
            if let Some(id) = state.id_relics[name as usize] {
                let relic = &mut state.entities[id];
                if relic.relic_counter >= 0 {
                    relic.relic_counter += 1;
                    if relic.relic_counter == relic.relic_counter_reset {
                        // Use -1 so that it doesn't proc again
                        relic.relic_counter = -1;
                        for &effect in relic.relic_effects_counter {
                            state.effect_buf.push(effect);
                        }
                    }
                }
            }
        }

        // Turn-start Relic effects (Mercury Hourglass); post-draw ones wait for the draw
        for &id_relic in &id_relics {
            let relic = &state.entities[id_relic];
            if !RELICS_TURN_START_POST_DRAW.contains(&relic.relic_name) {
                for &effect in relic.relic_effects_turn_start {
                    state.effect_buf.push(effect);
                }
            }
        }

        // Next turn energy: apply and clear
        if has_modifier(&modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(&modifiers, ModifierKind::NextTurnEnergy);
            state.effect_buf.push(Effect {
                kind: EffectKind::EnergyDelta {
                    sign: DeltaSign::Gain,
                    amount: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            modifier_remove(
                &mut state.entities[id_actor].modifiers,
                ModifierKind::NextTurnEnergy,
            );
        }

        // Choke auto-removes at the next player turn start
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierRemove {
                    kind: ModifierKind::Choke,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }

        // Spawn nightmare copies
        if id_card_nightmare.is_some() {
            state.effect_buf.push(Effect {
                kind: EffectKind::CardNightmareSpawn,
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Infinite blades: add `stacks` Shivs
        if has_modifier(&modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(&modifiers, ModifierKind::InfiniteBlades);
            state.effect_buf.push(Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Shiv,
                    pile: CardPile::Hand,
                    count: stacks.max(0) as u16,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Magnetism: add `stacks` random colorless Cards
        if has_modifier(&modifiers, ModifierKind::Magnetism) {
            let stacks = modifier_stacks(&modifiers, ModifierKind::Magnetism);
            state.effect_buf.push(Effect {
                kind: EffectKind::CardAddRandom {
                    color: CardColor::Colorless,
                    kind: None,
                    pile: CardPile::Hand,
                    count: stacks.max(0) as u8,
                    cost_zero: None,
                    upgraded: false,
                    rarity: None,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // recharge(): energy is set to max, not topped up; Ice Cream adds a full bar instead
        energy.energy_current = if has_relic(&state.id_relics, RelicName::IceCream) {
            energy.energy_current + energy.energy_max
        } else {
            energy.energy_max
        };

        // Hand size: 5, Snecko Eye +2, Ring of the Serpent +1
        let mut draw_count = CARDS_DRAWN_PER_TURN;
        if has_relic(&state.id_relics, RelicName::SneckoEye) {
            draw_count += 2;
        }
        if has_relic(&state.id_relics, RelicName::RingOfTheSerpent) {
            draw_count += 1;
        }

        // initializeDeck: innate and bottled Cards past the hand size draw extra on turn 1
        if first_turn {
            let n_top = id_card_draw
                .iter()
                .filter(|&&id| {
                    let card = &state.entities[id];
                    card.card_innate || card.card_bottled
                })
                .count() as u16;
            if n_top > draw_count {
                state.effect_buf.push(Effect {
                    kind: EffectKind::CardDraw {
                        count: n_top - draw_count,
                    },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }

        // Turn 1: pre-draw combat-start Relics (Ninja Scroll, Enchiridion)
        if first_turn {
            for &id_relic in &id_relics {
                let relic = &state.entities[id_relic];
                if RELICS_COMBAT_START_PRE_DRAW.contains(&relic.relic_name) {
                    for &effect in relic.relic_effects_combat_start {
                        state.effect_buf.push(effect);
                    }
                }
            }
        }

        // Organic Card draw
        state.effect_buf.push(Effect {
            kind: EffectKind::CardDraw { count: draw_count },
            id_source: None,
            target: Target::Direct(None),
        });

        // Turn 1: addToBot combat-start Relics, acquisition order
        if first_turn {
            for &id_relic in &id_relics {
                let relic = &state.entities[id_relic];
                let name = relic.relic_name;
                if !RELICS_COMBAT_START_TOP.contains(&name)
                    && !RELICS_COMBAT_START_PRE_DRAW.contains(&name)
                    && !RELICS_TURN_START_POST_DRAW.contains(&name)
                {
                    for &effect in relic.relic_effects_combat_start {
                        state.effect_buf.push(effect);
                    }
                }
            }
        }

        // Post-draw Relics (Warped Tongs; Gambling Chip's first-turn Gamble)
        for &id_relic in &id_relics {
            let relic = &state.entities[id_relic];
            if RELICS_TURN_START_POST_DRAW.contains(&relic.relic_name) {
                for &effect in relic.relic_effects_turn_start {
                    state.effect_buf.push(effect);
                }
                if first_turn {
                    for &effect in relic.relic_effects_combat_start {
                        state.effect_buf.push(effect);
                    }
                }
            }
        }

        // Noxius Fumes: Monsters get `stacks` poison stacks
        if has_modifier(&modifiers, ModifierKind::NoxiousFumes) {
            let stacks = modifier_stacks(&modifiers, ModifierKind::NoxiousFumes);
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Poison,
                        stacks,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }

        // Draw Cards next turn (Predator, Pocketwatch): apply and clear
        if has_modifier(&modifiers, ModifierKind::DrawCardNextTurn) {
            let stacks = modifier_stacks(&modifiers, ModifierKind::DrawCardNextTurn);
            state.effect_buf.push(Effect {
                kind: EffectKind::CardDraw {
                    count: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierRemove {
                    kind: ModifierKind::DrawCardNextTurn,
                },
                id_source: None,
                target: Target::Direct(Some(id_actor)),
            });
        }

        // Tools of the trade: draw `stacks`, discard `stacks`
        if has_modifier(&modifiers, ModifierKind::ToolsOfTheTrade) {
            let stacks = modifier_stacks(&modifiers, ModifierKind::ToolsOfTheTrade);
            state.effect_buf.push(Effect {
                kind: EffectKind::CardDraw {
                    count: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            state.effect_buf.push(Effect {
                kind: EffectKind::CardDiscard {
                    source: DiscardSource::Explicit, // Triggers on-discard sinergies
                },
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Hand,
                    filter: CandidateFilter::Any,
                    selection_kind: SelectionKind::Input {
                        count: stacks.max(0) as u16,
                    },
                },
            });
        }

        // Mayhem: PlayTopCardAction is queued when its wrapper runs, behind everything above
        if has_modifier(&modifiers, ModifierKind::Mayhem) {
            let stacks = modifier_stacks(&modifiers, ModifierKind::Mayhem);
            for _ in 0..stacks.max(0) {
                state.effect_buf.push(Effect {
                    kind: EffectKind::CardPlayFromDrawTop,
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
    }

    flush_effects_from_buf_to_queue_front(state);
}
