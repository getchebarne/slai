use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_set_not_new;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::types::Combat;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::shuffle;

// The Character's turn ends in two passes: its Relics, Plated Armor and self-playing Cards now; its other Modifiers and the discard land behind what those set off
pub fn process_effect_turn_end_character(state: &mut GameState, landing: bool) {
    assert!(
        state.combat.active,
        "process_effect_turn_end_character outside the Combat frame"
    );

    if !landing {
        let Combat {
            id_card_hand,
            id_card_draw,
            id_card_discard,
            id_card_exhaust,
            id_card_stasis,
            this_turn_discards,
            ..
        } = &mut state.combat;

        // Clear per-turn Card cost overrides
        for id_card in id_card_hand
            .iter()
            .chain(id_card_draw.iter())
            .chain(id_card_discard.iter())
            .chain(id_card_exhaust.iter())
            .chain(id_card_stasis.iter().flatten())
        {
            let entity = &mut state.entities[*id_card];
            if matches!(
                entity.card_cost_override,
                Some(CostOverride {
                    scope: CostScope::Turn,
                    ..
                })
            ) {
                entity.card_cost_override = None;
            }
        }

        // Clear effect buffer. The Relic and Card effects below go through effect_buf so they
        // resolve in order, ahead of the landing pass
        state.effect_buf.clear();

        // What the turn applied stops being new before Doubt and Shame land; TurnMonsters re-clears Vulnerable
        modifier_set_not_new(&mut state.entities[state.id_character].modifiers);

        // Orichalcum: Character gains 6 block if it has none, ahead of the other turn-end Relics
        if state.entities[state.id_character].vitals.block == 0
            && has_relic(&state.id_relics, RelicName::Orichalcum)
        {
            state.effect_buf.push(Effect {
                kind: EffectKind::BlockGain { amount: 6 },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }

        // Turn-end Relic effects, in acquisition order (Stone Calendar's damage, Nilry's Codex discover)
        let mut id_relics: Vec<usize> = state.id_relics.iter().flatten().copied().collect();
        id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);

        for id_relic in id_relics {
            let relic = &state.entities[id_relic];

            // Stone Calendar: fires once, at the end of the turn its counter reaches the reset threshold (turn 7)
            if relic.relic_name == RelicName::StoneCalendar
                && relic.relic_counter == relic.relic_counter_reset
            {
                for &effect in relic.relic_effects_counter {
                    state.effect_buf.push(effect);
                }
            }
            for &effect in relic.relic_effects_turn_end {
                state.effect_buf.push(effect);
            }
        }

        let mods_char = &state.entities[state.id_character].modifiers;

        // Plated Armor: gain block equal to stacks
        if has_modifier(mods_char, ModifierKind::PlatedArmor) {
            let stacks = modifier_stacks(mods_char, ModifierKind::PlatedArmor);
            state.effect_buf.push(Effect {
                kind: EffectKind::BlockGain {
                    amount: stacks as u16,
                },
                id_source: Some(state.id_character),
                target: Target::Direct(Some(state.id_character)),
            });
        }

        // Burn / Decay / Regret / Doubt / Shame play themselves out of hand
        for &id_card in id_card_hand.iter() {
            let card = &state.entities[id_card];
            match card.card_name {
                CardName::Burn => {
                    let damage: u16 = if card.card_upgraded { 4 } else { 2 };
                    state.effect_buf.push(Effect {
                        kind: EffectKind::DamageDeal {
                            amount: damage,
                            lifesteal: false,
                        },
                        id_source: None,
                        target: Target::Direct(Some(state.id_character)),
                    });
                }
                CardName::Decay => {
                    state.effect_buf.push(Effect {
                        kind: EffectKind::DamageDeal {
                            amount: 2,
                            lifesteal: false,
                        },
                        id_source: None,
                        target: Target::Direct(Some(state.id_character)),
                    });
                }
                CardName::Regret => {
                    state.effect_buf.push(Effect {
                        kind: EffectKind::HealthDelta {
                            sign: DeltaSign::Loss,
                            amount: Amount::Absolute(id_card_hand.len() as u16),
                        },
                        id_source: None,
                        target: Target::Direct(Some(state.id_character)),
                    });
                }
                CardName::Doubt => {
                    state.effect_buf.push(Effect {
                        kind: EffectKind::ModifierDelta {
                            kind: ModifierKind::Weak,
                            stacks: 1,
                        },
                        id_source: None,
                        target: Target::Direct(Some(state.id_character)),
                    });
                }
                CardName::Shame => {
                    state.effect_buf.push(Effect {
                        kind: EffectKind::ModifierDelta {
                            kind: ModifierKind::Frail,
                            stacks: 1,
                        },
                        id_source: None,
                        target: Target::Direct(Some(state.id_character)),
                    });
                }
                _ => continue,
            }
            state.effect_buf.push(Effect {
                kind: EffectKind::CardPlayRelocate,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }

        // The end-of-turn Modifiers and the discard wait for the Cards above
        state.effect_buf.push(Effect {
            kind: EffectKind::TurnEndCharacter { landing: true },
            id_source: None,
            target: Target::Direct(None),
        });

        // Reset per-turn trackers; attacks, plays and Panache's countdown run on until the next turn start
        *this_turn_discards = 0;

        flush_effects_from_buf_to_queue_front(state);
        return;
    }

    let Combat {
        id_card_hand,
        bombs,
        ..
    } = &state.combat;
    let mods_char = &state.entities[state.id_character].modifiers;

    // Regeneration: heal `stacks`, then decrement by 1 (removed at 0); ahead of every other Modifier
    if has_modifier(mods_char, ModifierKind::Regeneration) {
        let stacks = modifier_stacks(mods_char, ModifierKind::Regeneration);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(stacks.max(0) as u16),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Regeneration,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Retain: pick up to `stacks` Cards to keep through the end-of-turn discard
    if has_modifier(mods_char, ModifierKind::Retain)
        && !id_card_hand.is_empty()
        // Runic Pyramid: keeps the whole hand
        && !has_relic(&state.id_relics, RelicName::RunicPyramid)
    {
        let stacks = modifier_stacks(mods_char, ModifierKind::Retain);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardRetain,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                filters: &[],
                selection_kind: SelectionKind::InputUpTo {
                    count: stacks.max(0) as u16,
                },
            },
        });
    }

    // Ritual: gain `stacks` Strength each turn end; a Monster's Ritual gains at the round end instead
    if has_modifier(mods_char, ModifierKind::Ritual) {
        let stacks = modifier_stacks(mods_char, ModifierKind::Ritual);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Wraith Form: lose `stacks` Dexterity each turn end, ahead of LoseStrength and LoseDexterity
    if has_modifier(mods_char, ModifierKind::WraithForm) {
        let stacks = modifier_stacks(mods_char, ModifierKind::WraithForm);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Dexterity,
                stacks: -stacks,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Lose{Strength,Dexterity}: the borrowed stacks leave at turn end
    for (lose, gain) in [
        (ModifierKind::LoseStrength, ModifierKind::Strength),
        (ModifierKind::LoseDexterity, ModifierKind::Dexterity),
    ] {
        if has_modifier(mods_char, lose) {
            let stacks = modifier_stacks(mods_char, lose);
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierDelta {
                    kind: gain,
                    stacks: -stacks,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierRemove { kind: lose },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // Queue `EffectKind::ModifierRemove`s for Modifiers that clear at end of turn
    for kind in [
        ModifierKind::Burst,
        ModifierKind::NoDraw,
        ModifierKind::Entangled,
    ] {
        if has_modifier(mods_char, kind) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierRemove { kind },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // DuplicateNextCardPlay ticks down one stack; a last stack is removed, never left at 0
    if has_modifier(mods_char, ModifierKind::DuplicateNextCardPlay) {
        let effect_kind = if modifier_stacks(mods_char, ModifierKind::DuplicateNextCardPlay) > 1 {
            EffectKind::ModifierDelta {
                kind: ModifierKind::DuplicateNextCardPlay,
                stacks: -1,
            }
        } else {
            EffectKind::ModifierRemove {
                kind: ModifierKind::DuplicateNextCardPlay,
            }
        };
        state.effect_queue.push_back(Effect {
            kind: effect_kind,
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Bombs count down when their slot comes, so the Retain pick still shows the old fuses
    if !bombs.is_empty() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::BombTick,
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Ethereal Cards exhaust first, in random order, each spending its free play; Cards drawn after this point stay in hand
    let mut id_ethereal: Vec<usize> = id_card_hand
        .iter()
        .copied()
        .filter(|&id| state.entities[id].card_ethereal)
        .collect();
    shuffle(&mut id_ethereal, &mut state.rng);
    for id_card in id_ethereal {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardExhaust,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardFreePlaySpend,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // The rest leave from the right, so the leftmost ends on top of the discard pile; Runic Pyramid keeps them
    if !has_relic(&state.id_relics, RelicName::RunicPyramid) {
        for &id_card in id_card_hand.iter().rev() {
            if state.entities[id_card].card_ethereal {
                continue;
            }
            state.effect_queue.push_back(Effect {
                kind: EffectKind::CardDiscard {
                    source: DiscardSource::EndOfTurn,
                },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }

    // The Monsters' turns wait for the exhausts and discards and what they set off (Dead Branch's Card)
    state.effect_queue.push_back(Effect {
        kind: EffectKind::TurnMonsters,
        id_source: None,
        target: Target::Direct(None),
    });
}
