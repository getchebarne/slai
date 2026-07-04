use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::relics::relic_counter_fire;
use crate::types::CardName;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_turn_start(id_target: Option<usize>, state: &mut GameState) {
    let id_actor = id_target.expect("TurnStart requires id_target");
    let id_character = state.id_character;
    let energy_max = state.energy.energy_max;
    let energy_current = state.energy.energy_current;
    let nightmare_pending = state.id_card_nightmare.is_some();
    let id_monsters = state.id_monsters;

    state.effect_buf.clear();

    let entity = &mut state.entities[id_actor];
    let modifiers = &mut entity.modifiers;
    let vitals = &mut entity.vitals;

    if modifier_has(modifiers, ModifierKind::Poison) {
        state.effect_buf.push(Effect {
            kind: EffectKind::PoisonTick,
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    let mut new_block: u16 = 0;
    if modifier_has(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    // Calipers: retain block minus 15 instead of losing all; max with Blur, never additive
    if id_actor == id_character && state.id_relics[RelicName::Calipers as usize].is_some() {
        new_block = new_block.max(vitals.block.saturating_sub(15));
    }
    if modifier_has(modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(modifiers, ModifierKind::NextTurnBlock);
    }
    state.effect_buf.push(Effect {
        kind: EffectKind::BlockSet { amount: new_block },
        id_source: None,
        target: Target::Direct(Some(id_actor)),
    });

    if modifier_has(modifiers, ModifierKind::Phantasmal) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DoubleDamage,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    if id_actor == id_character {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardDraw {
                count: CARDS_DRAWN_PER_TURN,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        // Ice Cream: refill adds a full energy_max on top instead of topping up
        let energy_gain = if state.id_relics[RelicName::IceCream as usize].is_some() {
            energy_max
        } else {
            energy_max.saturating_sub(energy_current)
        };
        state.effect_buf.push(Effect {
            kind: EffectKind::EnergyGain {
                amount: energy_gain as u16,
            },
            id_source: None,
            target: Target::Direct(None),
        });

        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierTick,
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierTick,
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }

        if modifier_has(modifiers, ModifierKind::NoxiousFumes) {
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

        if nightmare_pending {
            state.effect_buf.push(Effect {
                kind: EffectKind::CardNightmareSpawn,
                id_source: None,
                target: Target::Direct(None),
            });
        }

        if modifier_has(modifiers, ModifierKind::DrawCardNextTurn) {
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

        if modifier_has(modifiers, ModifierKind::ToolsOfTheTrade) {
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
                    source: DiscardSource::Explicit,
                },
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Hand,
                    selection_kind: SelectionKind::Input {
                        count: stacks.max(0) as u16,
                    },
                },
            });
        }

        if modifier_has(modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NextTurnEnergy);
            state.effect_buf.push(Effect {
                kind: EffectKind::EnergyGain {
                    amount: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            modifier_remove(modifiers, ModifierKind::NextTurnEnergy);
        }

        if modifier_has(modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(modifiers, ModifierKind::InfiniteBlades);
            state.effect_buf.push(Effect {
                kind: EffectKind::CardAddToHand {
                    card_name: CardName::Shiv,
                    count: stacks.max(0) as u16,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Persistent turn counters, spanning combats
        if relic_counter_fire(RelicName::HappyFlower, 3, &state.id_relics, &mut state.entities) {
            state.effect_buf.push(Effect {
                kind: EffectKind::EnergyGain { amount: 1 },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        if relic_counter_fire(RelicName::IncenseBurner, 6, &state.id_relics, &mut state.entities) {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Intangible,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }

        // Thorns-type damage: id_source None skips Strength/Weak scaling and Envenom
        if state.id_relics[RelicName::MercuryHourglass as usize].is_some() {
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal { amount: 3 },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }

        // Once per combat at the Nth turn start; -1 parks the counter until combat-start reset
        for (name, turn_n, block) in [
            (RelicName::HornCleat, 2, 14),
            (RelicName::CaptainsWheel, 3, 18),
        ] {
            if let Some(id) = state.id_relics[name as usize] {
                let counter = &mut state.entities[id].relic_counter;
                if *counter >= 0 {
                    *counter += 1;
                    if *counter == turn_n {
                        *counter = -1;
                        // Relic-sourced block: id_source None skips Dex/Frail scaling
                        state.effect_buf.push(Effect {
                            kind: EffectKind::BlockGain { amount: block },
                            id_source: None,
                            target: Target::Direct(Some(id_character)),
                        });
                    }
                }
            }
        }
    }

    flush_effects_from_buf_to_queue_front(state);
}

#[cfg(test)]
mod tests {
    use crate::engine::test_support::char_modifier;
    use crate::engine::test_support::combat_with_relic;
    use crate::engine::test_support::end_turn;
    use crate::engine::test_support::first_monster;
    use crate::engine::test_support::set_relic_counter;
    use crate::modifier::ModifierKind;
    use crate::types::MonsterName;
    use crate::types::RelicName;

    #[test]
    fn happy_flower_grants_energy_every_third_turn() {
        let mut state = combat_with_relic(RelicName::HappyFlower, MonsterName::Cultist);
        let id = state.id_relics[RelicName::HappyFlower as usize].unwrap();
        // Turn 1 already counted during combat setup
        assert_eq!(state.entities[id].relic_counter, 1);
        end_turn(&mut state);
        assert_eq!(state.energy.energy_current, 3);
        end_turn(&mut state);
        // Turn 3: fire and reset
        assert_eq!(state.energy.energy_current, 4);
        assert_eq!(state.entities[id].relic_counter, 0);
    }

    #[test]
    fn incense_burner_grants_intangible_every_sixth_turn() {
        let mut state = combat_with_relic(RelicName::IncenseBurner, MonsterName::Cultist);
        set_relic_counter(&mut state, RelicName::IncenseBurner, 5);
        end_turn(&mut state);
        assert_eq!(char_modifier(&state, ModifierKind::Intangible), 1);
    }

    #[test]
    fn mercury_hourglass_damages_all_enemies_each_turn() {
        let mut state = combat_with_relic(RelicName::MercuryHourglass, MonsterName::Cultist);
        let id_monster = first_monster(&state);
        let hp_max = state.entities[id_monster].vitals.health_max;
        // Turn 1 tick happened during combat setup
        assert_eq!(state.entities[id_monster].vitals.health, hp_max - 3);
        end_turn(&mut state);
        assert_eq!(state.entities[id_monster].vitals.health, hp_max - 6);
    }

    #[test]
    fn horn_cleat_blocks_on_turn_two() {
        let mut state = combat_with_relic(RelicName::HornCleat, MonsterName::Cultist);
        assert_eq!(state.entities[state.id_character].vitals.block, 0);
        end_turn(&mut state);
        assert_eq!(state.entities[state.id_character].vitals.block, 14);
        let id = state.id_relics[RelicName::HornCleat as usize].unwrap();
        assert_eq!(state.entities[id].relic_counter, -1);
    }

    #[test]
    fn captains_wheel_blocks_once_on_turn_three() {
        let mut state = combat_with_relic(RelicName::CaptainsWheel, MonsterName::Cultist);
        end_turn(&mut state);
        assert_eq!(state.entities[state.id_character].vitals.block, 0);
        end_turn(&mut state);
        assert_eq!(state.entities[state.id_character].vitals.block, 18);
        end_turn(&mut state);
        // Turn 4: parked at -1, no refire
        assert_eq!(state.entities[state.id_character].vitals.block, 0);
    }

    #[test]
    fn calipers_retains_block_minus_fifteen() {
        let mut state = combat_with_relic(RelicName::Calipers, MonsterName::Cultist);
        let id_character = state.id_character;
        state.entities[id_character].vitals.block = 40;
        end_turn(&mut state);
        assert_eq!(state.entities[id_character].vitals.block, 25);
    }

    #[test]
    fn ice_cream_carries_energy_over() {
        let mut state = combat_with_relic(RelicName::IceCream, MonsterName::Cultist);
        assert_eq!(state.energy.energy_current, 3);
        end_turn(&mut state);
        assert_eq!(state.energy.energy_current, 6);
    }
}
