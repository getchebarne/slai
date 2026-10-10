use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::consts::ENERGY_CAP;
use crate::consts::PANACHE_PLAYS;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::entity::Entity;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::PRIORITY_DEFAULT;
use crate::modifier::has_modifier;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::relics::RELIC_COUNTERS_PER_TURN;
use crate::relics::RELICS_COMBAT_START_FIRST;
use crate::relics::RELICS_COMBAT_START_PRE_DRAW;
use crate::relics::RELICS_COMBAT_START_TOP;
use crate::relics::RELICS_TURN_START_POST_DRAW;
use crate::relics::trigger_relic_counter;
use crate::types::CardColor;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Combat;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::hook_order;

// Character's turn start; turn 1 also slots in the combat-start Relics
pub fn process_effect_turn_start_character(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_turn_start_character outside the Combat frame"
    );

    // The Character's turn is under way again
    state.combat.turn_ended = false;

    let Combat {
        id_monsters,
        id_card_pile_hand,
        id_card_pile_draw,
        id_card_pile_discard,
        energy,
        id_card_pile_nightmare,
        turn,
        this_turn_attacks,
        this_turn_cards_played,
        this_turn_discards,
        orange_pellets_played_attack,
        orange_pellets_played_skill,
        orange_pellets_played_power,
        panache_countdown,
        ..
    } = &mut state.combat;

    // Clear effect buffer
    state.effect_buf.clear();

    // Get mutable references
    let character = &mut state.entities[state.id_character];
    let modifiers = &mut character.modifiers;
    let vitals = &mut character.vitals;

    // Blur: existing block skips turn-stat reset
    let mut new_block: u16 = 0;
    if has_modifier(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }

    // Calipers: retain block minus 15 instead of losing all; max with Blur, never additive
    if has_relic(&state.id_relics, RelicName::Calipers) {
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
        target: Target::Direct(Some(state.id_character)),
    });

    // Phantasmal: gains double damage
    if has_modifier(modifiers, ModifierKind::Phantasmal) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::DoubleDamage,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    let first_turn = *turn == 0;
    *turn += 1;
    let modifiers = state.entities[state.id_character].modifiers;

    // Last turn's plays decide Art of War and Pocketwatch; this turn counts from zero
    let art_of_war_energy = !first_turn && *this_turn_attacks == 0;
    let pocketwatch_draw = !first_turn
        && state.id_relics[RelicName::Pocketwatch as usize]
            .is_some_and(|id| state.entities[id].relic_counter <= 3);
    *this_turn_attacks = 0;
    *this_turn_cards_played = 0;
    *this_turn_discards = 0;
    *orange_pellets_played_attack = false;
    *orange_pellets_played_skill = false;
    *orange_pellets_played_power = false;
    *panache_countdown = PANACHE_PLAYS;
    for &name in RELIC_COUNTERS_PER_TURN {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = 0;
        }
    }

    // Eviscerate's cost this turn restarts at its combat cost; one the Monsters' half returned was priced by last turn's discards
    for &id_card in id_card_pile_draw
        .iter()
        .chain(id_card_pile_hand.iter())
        .chain(id_card_pile_discard.iter())
    {
        let card = &mut state.entities[id_card];
        if card.card_cost_kind == CardCostKind::MinusDiscardsThisTurn
            && matches!(
                card.card_cost_override,
                Some(CostOverride {
                    scope: CostScope::Turn,
                    ..
                })
            )
        {
            card.card_cost_override = None;
        }
    }

    // Owned Relics in acquisition order
    let mut id_relics: Vec<usize> = state.id_relics.iter().flatten().copied().collect();
    id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);

    // Hand size: 5, Snecko Eye +2, Ring of the Serpent +1
    let mut draw_count = CARDS_DRAWN_PER_TURN;
    if has_relic(&state.id_relics, RelicName::SneckoEye) {
        draw_count += 2;
    }
    if has_relic(&state.id_relics, RelicName::RingOfTheSerpent) {
        draw_count += 1;
    }

    if first_turn {
        // Turn 1: the first combat-start Relics (Enchiridion, Snecko Eye)
        for &id_relic in &id_relics {
            let relic = &state.entities[id_relic];
            if RELICS_COMBAT_START_FIRST.contains(&relic.relic_name) {
                for &effect in relic.relic_effects_combat_start {
                    state.effect_buf.push(effect);
                }
            }
        }

        // Innate and bottled Cards past the hand size draw extra, after the first Relics
        let num_top = id_card_pile_draw
            .iter()
            .filter(|&&id| {
                let card = &state.entities[id];
                card.card_innate || card.card_bottled
            })
            .count() as u16;
        if num_top > draw_count {
            state.effect_buf.push(Effect {
                kind: EffectKind::CardDraw {
                    count: num_top - draw_count,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Front-queued combat-start Relics, newest pickup first
        for &id_relic in id_relics.iter().rev() {
            let relic = &state.entities[id_relic];
            if RELICS_COMBAT_START_TOP.contains(&relic.relic_name) {
                for &effect in relic.relic_effects_combat_start {
                    state.effect_buf.push(effect);
                }
            }
        }
    } else {
        // Round end: duration Modifiers tick (Character's, then Monsters')
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierTick,
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });

        // DuplicateNextCardPlay ticks down one stack at the round end; a last stack is removed, never left at 0
        if has_modifier(&modifiers, ModifierKind::DuplicateNextCardPlay) {
            let kind = if modifier_stacks(&modifiers, ModifierKind::DuplicateNextCardPlay) > 1 {
                EffectKind::ModifierDelta {
                    kind: ModifierKind::DuplicateNextCardPlay,
                    stacks: -1,
                }
            } else {
                EffectKind::ModifierRemove {
                    kind: ModifierKind::DuplicateNextCardPlay,
                }
            };
            state.effect_buf.push(Effect {
                kind,
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierTick,
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }

        // Turn-start Relics run before the draw; turn 1 defers them past the back-queued Relics
        push_relics_turn_start(
            &id_relics,
            &mut state.entities,
            &mut state.effect_buf,
            art_of_war_energy,
            *turn,
        );
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
            &mut state.entities[state.id_character].modifiers,
            ModifierKind::NextTurnEnergy,
        );
    }

    // The Card hooks below fire by priority, then by stamp, so the hand fills in the order they appeared
    let mut hooks: Vec<((u8, u32), Effect)> = Vec::new();

    // Nightmare: each pending snapshot is its own hook and adds its copies
    for &(id_snapshot, seq) in id_card_pile_nightmare.iter() {
        hooks.push((
            (PRIORITY_DEFAULT, seq),
            Effect {
                kind: EffectKind::CardNightmareSpawn,
                id_source: None,
                target: Target::Direct(Some(id_snapshot)),
            },
        ));
    }

    // Infinite blades: add `stacks` Shivs
    if has_modifier(&modifiers, ModifierKind::InfiniteBlades) {
        let stacks = modifier_stacks(&modifiers, ModifierKind::InfiniteBlades);
        hooks.push((
            hook_order(&modifiers, ModifierKind::InfiniteBlades),
            Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Shiv,
                    card_pile: CardPile::Hand,
                    count: stacks.max(0) as u16,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            },
        ));
    }

    // Magnetism: add `stacks` random colorless Cards
    if has_modifier(&modifiers, ModifierKind::Magnetism) {
        let stacks = modifier_stacks(&modifiers, ModifierKind::Magnetism);
        hooks.push((
            hook_order(&modifiers, ModifierKind::Magnetism),
            Effect {
                kind: EffectKind::CardAddRandom {
                    color: CardColor::Colorless,
                    kind: None,
                    card_pile: CardPile::Hand,
                    count: stacks.max(0) as u16,
                    cost_zero: None,
                    upgraded: false,
                    rarity: None,
                },
                id_source: None,
                target: Target::Direct(None),
            },
        ));
    }

    // Mayhem: each stack picks a living Monster before the draw; MayhemProc queues the play
    if has_modifier(&modifiers, ModifierKind::Mayhem) {
        let stacks = modifier_stacks(&modifiers, ModifierKind::Mayhem);
        for _ in 0..stacks.max(0) {
            hooks.push((
                hook_order(&modifiers, ModifierKind::Mayhem),
                Effect {
                    kind: EffectKind::MayhemProc,
                    id_source: None,
                    target: Target::Resolve {
                        candidate_pool: CandidatePool::Monsters,
                        filters: &[],
                        selection_kind: SelectionKind::Random { count: 1 },
                    },
                },
            ));
        }
    }

    // A stable sort keeps each hook's own effects in order
    hooks.sort_by_key(|&(order, _)| order);
    state
        .effect_buf
        .extend(hooks.into_iter().map(|(_, effect)| effect));

    // Energy resets to max; turn 1 and Ice Cream add a full bar instead
    energy.energy_current = if first_turn || has_relic(&state.id_relics, RelicName::IceCream) {
        (energy.energy_current + energy.energy_max).min(ENERGY_CAP)
    } else {
        energy.energy_max
    };

    // Turn 1: pre-draw combat-start Relics (Ninja Scroll, Toolbox)
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

    if first_turn {
        // Turn 1: back-queued combat-start Relics, acquisition order
        for &id_relic in &id_relics {
            let relic = &state.entities[id_relic];
            let name = relic.relic_name;
            if !RELICS_COMBAT_START_TOP.contains(&name)
                && !RELICS_COMBAT_START_FIRST.contains(&name)
                && !RELICS_COMBAT_START_PRE_DRAW.contains(&name)
                && !RELICS_TURN_START_POST_DRAW.contains(&name)
            {
                for &effect in relic.relic_effects_combat_start {
                    state.effect_buf.push(effect);
                }
            }
        }

        // Pen Nib: a charge primed last combat (counter 9) re-applies after the opening draw
        if let Some(id) = state.id_relics[RelicName::PenNib as usize]
            && state.entities[id].relic_counter == 9
        {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::PenNib,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }

        // Turn 1: turn-start Relics follow the back-queued combat-start Relics and Pen Nib
        push_relics_turn_start(
            &id_relics,
            &mut state.entities,
            &mut state.effect_buf,
            art_of_war_energy,
            *turn,
        );
    }

    // Post-draw Relics (Warped Tongs; Gambling Chip's first-turn Gamble)
    for &id_relic in &id_relics {
        let relic = &state.entities[id_relic];

        // Pocketwatch: 3 or fewer Cards played last turn draws 3 more
        if relic.relic_name == RelicName::Pocketwatch && pocketwatch_draw {
            state.effect_buf.push(Effect {
                kind: EffectKind::CardDraw { count: 3 },
                id_source: None,
                target: Target::Direct(None),
            });
        }
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

    // The post-draw hooks' priorities all differ, so this fixed order is their priority order

    // Noxius Fumes: the Character gives Monsters `stacks` poison stacks
    if has_modifier(&modifiers, ModifierKind::NoxiousFumes) {
        let stacks = modifier_stacks(&modifiers, ModifierKind::NoxiousFumes);
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Poison,
                    stacks,
                },
                id_source: Some(state.id_character),
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // Draw Cards next turn (Predator, Doppelganger): apply and clear
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
            target: Target::Direct(Some(state.id_character)),
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
                candidate_pool: CandidatePool::CardPileHand,
                filters: &[],
                selection_kind: SelectionKind::Input {
                    count: stacks.max(0) as u16,
                },
            },
        });
    }

    flush_effects_from_buf_to_queue_front(state);
}

// Turn-start Relics in acquisition order: each advances its counter and queues what fires; post-draw ones wait for the draw
fn push_relics_turn_start(
    id_relics_by_seq: &[usize],
    entities: &mut [Entity],
    effect_buf: &mut Vec<Effect>,
    art_of_war_energy: bool,
    turn: u16,
) {
    for &id_relic in id_relics_by_seq {
        let relic = &mut entities[id_relic];
        match relic.relic_name {
            // Persistent turn counters, spanning combats
            RelicName::HappyFlower | RelicName::IncenseBurner => {
                if trigger_relic_counter(relic) {
                    effect_buf.extend_from_slice(relic.relic_effects_counter);
                }
            }

            // Stone Calendar: counts the combat's turns; it fires at the turn end
            RelicName::StoneCalendar => relic.relic_counter += 1,

            // Horn Cleat and Captain's Wheel: count the combat's turns up to their turn, fire there once, then read 0
            RelicName::HornCleat | RelicName::CaptainsWheel => {
                if turn <= relic.relic_counter_reset as u16 && trigger_relic_counter(relic) {
                    effect_buf.extend_from_slice(relic.relic_effects_counter);
                }
            }

            // Art of War: a turn without Attacks gives 1 energy
            RelicName::ArtOfWar if art_of_war_energy => effect_buf.push(Effect {
                kind: EffectKind::EnergyDelta {
                    sign: DeltaSign::Gain,
                    amount: 1,
                },
                id_source: None,
                target: Target::Direct(None),
            }),
            _ => {}
        }

        // Turn-start Relic effects (Mercury Hourglass)
        if !RELICS_TURN_START_POST_DRAW.contains(&relic.relic_name) {
            effect_buf.extend_from_slice(relic.relic_effects_turn_start);
        }
    }
}
