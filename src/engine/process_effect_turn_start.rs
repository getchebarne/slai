use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
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
use crate::relics::trigger_relic_counter;
use crate::types::CardColor;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

pub fn process_effect_turn_start(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat {
        id_monsters,
        energy,
        id_card_nightmare,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_turn_start outside Combat mode")
    };
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

    // Character's turn start
    if id_actor == state.id_character {
        // Mayhem: autoplay `stacks` cards off the top, before the turn's draw
        if has_modifier(modifiers, ModifierKind::Mayhem) {
            let stacks = modifier_stacks(modifiers, ModifierKind::Mayhem);
            for _ in 0..stacks.max(0) {
                state.effect_buf.push(Effect {
                    kind: EffectKind::CardPlayFromDrawTop,
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }

        // Organic card draw
        state.effect_buf.push(Effect {
            kind: EffectKind::CardDraw {
                count: CARDS_DRAWN_PER_TURN,
            },
            id_source: None,
            target: Target::Direct(None),
        });

        // Ice Cream: refill adds a full energy_max on top instead of topping up
        let energy_gain = if has_relic(&state.id_relics, RelicName::IceCream) {
            energy.energy_max
        } else {
            energy.energy_max.saturating_sub(energy.energy_current)
        };

        // Energy refill
        state.effect_buf.push(Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: energy_gain as u16,
            },
            id_source: None,
            target: Target::Direct(None),
        });

        // Modifier tick (Character's and Monsters')
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

        // Noxius Fumes: Monsters get `stacks` poison stacks
        if has_modifier(modifiers, ModifierKind::NoxiousFumes) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NoxiousFumes);
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

        // Draw cards next turn (Predator, Pocketwatch): apply and clear
        if has_modifier(modifiers, ModifierKind::DrawCardNextTurn) {
            let stacks = modifier_stacks(modifiers, ModifierKind::DrawCardNextTurn);
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
        if has_modifier(modifiers, ModifierKind::ToolsOfTheTrade) {
            let stacks = modifier_stacks(modifiers, ModifierKind::ToolsOfTheTrade);
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
                    candidate_pool: CandidatePool::Hand {
                        filter: CandidatePoolCardFilter::Any,
                    },
                    selection_kind: SelectionKind::Input {
                        count: stacks.max(0) as u16,
                    },
                },
            });
        }

        // Next turn energy: apply and clear
        if has_modifier(modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NextTurnEnergy);
            state.effect_buf.push(Effect {
                kind: EffectKind::EnergyDelta {
                    sign: DeltaSign::Gain,
                    amount: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            modifier_remove(modifiers, ModifierKind::NextTurnEnergy);
        }

        // Infinite blades: add `stacks` Shivs
        if has_modifier(modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(modifiers, ModifierKind::InfiniteBlades);
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

        // Magnetism: add `stacks` random colorless cards
        if has_modifier(modifiers, ModifierKind::Magnetism) {
            let stacks = modifier_stacks(modifiers, ModifierKind::Magnetism);
            state.effect_buf.push(Effect {
                kind: EffectKind::CardAddRandom {
                    color: CardColor::Colorless,
                    kind: None,
                    pile: CardPile::Hand,
                    count: stacks.max(0) as u8,
                    cost_zero: None,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Persistent turn counters, spanning combats
        // Happy Flower: gain +1 energy every 3 turns
        if trigger_relic_counter(
            RelicName::HappyFlower,
            3,
            &state.id_relics,
            &mut state.entities,
        ) {
            state.effect_buf.push(Effect {
                kind: EffectKind::EnergyDelta {
                    sign: DeltaSign::Gain,
                    amount: 1,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Incense Burner: gain +1 `ModifierKind::Intangible` every 6 turns
        if trigger_relic_counter(
            RelicName::IncenseBurner,
            6,
            &state.id_relics,
            &mut state.entities,
        ) {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Intangible,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }

        // Mercury Hourglass: deal 3 damage to all Monsters
        if has_relic(&state.id_relics, RelicName::MercuryHourglass) {
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal { amount: 3 },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }

        // Horn Cleat and Captain's Wheel: gain block at the 2nd and 3rd turns, respectively
        // TODO: add combat turn # field to `GameState`
        for (name, turn_num, block) in [
            (RelicName::HornCleat, 2, 14),
            (RelicName::CaptainsWheel, 3, 18),
        ] {
            if let Some(id) = state.id_relics[name as usize] {
                let counter = &mut state.entities[id].relic_counter;
                if *counter >= 0 {
                    *counter += 1;
                    if *counter == turn_num {
                        // Use -1 so that it doesn't proc again
                        *counter = -1;

                        // Relic-sourced block: id_source None skips Dex / Frail scaling
                        state.effect_buf.push(Effect {
                            kind: EffectKind::BlockGain { amount: block },
                            id_source: None,
                            target: Target::Direct(Some(state.id_character)),
                        });
                    }
                }
            }
        }

        // Warped Tongs: pushed after the draws so the pick sees the drawn hand
        if has_relic(&state.id_relics, RelicName::WarpedTongs) {
            state.effect_buf.push(Effect {
                kind: EffectKind::CardUpgrade,
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Hand {
                        filter: CandidatePoolCardFilter::Upgradeable,
                    },
                    selection_kind: SelectionKind::Random { count: 1 },
                },
            });
        }
    }

    flush_effects_from_buf_to_queue_front(state);
}
