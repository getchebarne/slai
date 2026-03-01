pub mod process_effect_add_shivs;
pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_discard;
pub mod process_effect_card_discard_all;
pub mod process_effect_card_draw;
pub mod process_effect_card_exhaust;
pub mod process_effect_card_play;
pub mod process_effect_card_remove;
pub mod process_effect_card_reward_clear;
pub mod process_effect_card_reward_roll;
pub mod process_effect_card_reward_select;
pub mod process_effect_card_upgrade;
pub mod process_effect_combat_end;
pub mod process_effect_combat_start;
pub mod process_effect_damage_deal;
pub mod process_effect_damage_physical;
pub mod process_effect_death;
pub mod process_effect_energy_gain;
pub mod process_effect_energy_loss;
pub mod process_effect_health_gain;
pub mod process_effect_health_loss;
pub mod process_effect_modifier_gain;
pub mod process_effect_modifier_remove;
pub mod process_effect_modifier_set_not_new;
pub mod process_effect_modifier_tick;
pub mod process_effect_move_update;
pub mod process_effect_room_enter;
pub mod process_effect_target_clear;
pub mod process_effect_target_set;
pub mod process_effect_turn_end;
pub mod process_effect_turn_start;

use crate::effect::{Effect, EffectTemplate, TargetKind};
use crate::state::{EntityKind, GameState};
use crate::types::EntityId;

pub enum ProcessEffectResult {
    Continue { top: Vec<Effect>, bot: Vec<Effect> },
    Pass,
    Halt,
    Pause,
}

pub fn resolve_target_kind(
    target_kind: TargetKind,
    source: EntityId,
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
) -> Vec<EntityId> {
    match target_kind {
        TargetKind::CardTarget => vec![card_target.unwrap()],
        TargetKind::Character => vec![EntityId(0)],
        TargetKind::AllMonsters => alive_monsters.to_vec(),
        TargetKind::Source => vec![source],
    }
}

pub fn instantiate_templates(
    templates: &[EffectTemplate],
    source: EntityId,
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
) -> Vec<Effect> {
    let mut out = Vec::new();
    for tmpl in templates {
        match *tmpl {
            EffectTemplate::DamagePhysical { base, target } => {
                for actor in resolve_target_kind(target, source, card_target, alive_monsters) {
                    out.push(Effect::DamagePhysical {
                        source,
                        target: actor,
                        base,
                    });
                }
            }
            EffectTemplate::BlockGain { amount, target } => {
                for actor in resolve_target_kind(target, source, card_target, alive_monsters) {
                    out.push(Effect::BlockGain {
                        target: actor,
                        amount,
                        from_card: true,
                    });
                }
            }
            EffectTemplate::ModifierGain {
                kind,
                stacks,
                target,
            } => {
                for actor in resolve_target_kind(target, source, card_target, alive_monsters) {
                    out.push(Effect::ModifierGain {
                        target: actor,
                        kind,
                        stacks,
                    });
                }
            }
            EffectTemplate::ModifierRemove { kind, target } => {
                for actor in resolve_target_kind(target, source, card_target, alive_monsters) {
                    out.push(Effect::ModifierRemove {
                        target: actor,
                        kind,
                    });
                }
            }
            EffectTemplate::EnergyGain { amount } => {
                out.push(Effect::EnergyGain { amount });
            }
            EffectTemplate::AddShivs { count } => {
                out.push(Effect::AddShivs { count });
            }
            EffectTemplate::CardDraw { count } => {
                out.push(Effect::CardDraw { count });
            }
            EffectTemplate::CardDiscardInput => {
                out.push(Effect::AwaitDiscard);
            }
            EffectTemplate::CardDiscardRandom => {
                out.push(Effect::CardDiscardAll);
            }
            EffectTemplate::CalculatedGamble => {
                out.push(Effect::CalculatedGamble);
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

pub fn process_effect(state: &mut GameState, effect: Effect) -> ProcessEffectResult {
    match effect {
        Effect::CardDraw { count } => process_effect_card_draw::process_effect_card_draw(
            count,
            &mut state.draw_pile,
            &mut state.hand,
            &mut state.discard_pile,
            &mut state.rng,
        ),
        Effect::CardPlay { card_id } => {
            let alive = state.alive_monster_ids();
            process_effect_card_play::process_effect_card_play(
                card_id,
                &state.entities,
                state.card_target,
                &alive,
            )
        }
        Effect::CardDiscard { card_id } => {
            process_effect_card_discard::process_effect_card_discard(
                card_id,
                &mut state.hand,
                &mut state.discard_pile,
            )
        }
        Effect::CardDiscardAll => process_effect_card_discard_all::process_effect_card_discard_all(
            &mut state.hand,
            &mut state.discard_pile,
        ),
        Effect::CardExhaust { card_id } => {
            process_effect_card_exhaust::process_effect_card_exhaust(
                card_id,
                &mut state.hand,
                &mut state.exhaust_pile,
            )
        }
        Effect::CardRemove { card_id } => {
            process_effect_card_remove::process_effect_card_remove(card_id, &mut state.hand)
        }
        Effect::AddShivs { count } => process_effect_add_shivs::process_effect_add_shivs(
            count,
            &mut state.entities,
            &mut state.hand,
            &mut state.discard_pile,
        ),
        Effect::CalculatedGamble => {
            process_effect_calculated_gamble::process_effect_calculated_gamble(&mut state.hand)
        }
        Effect::CardUpgrade { deck_idx } => {
            process_effect_card_upgrade::process_effect_card_upgrade(
                deck_idx,
                &state.deck,
                &mut state.entities,
            )
        }
        Effect::CardRewardRoll => {
            process_effect_card_reward_roll::process_effect_card_reward_roll(
                &mut state.card_rewards,
                &mut state.entities,
                &mut state.rng,
            )
        }
        Effect::CardRewardSelect { reward_idx } => {
            process_effect_card_reward_select::process_effect_card_reward_select(
                reward_idx,
                &mut state.card_rewards,
                &mut state.deck,
            )
        }
        Effect::CardRewardClear => {
            process_effect_card_reward_clear::process_effect_card_reward_clear(
                &mut state.card_rewards,
            )
        }
        Effect::TargetSet { target } => {
            process_effect_target_set::process_effect_target_set(&mut state.card_target, target)
        }
        Effect::TargetClear => {
            process_effect_target_clear::process_effect_target_clear(&mut state.card_target)
        }
        Effect::DamagePhysical {
            source,
            target,
            base,
        } => {
            let (_, source_mods) = state.entities[source.0 as usize].kind.combatant_ref();
            let (_, target_mods) = state.entities[target.0 as usize].kind.combatant_ref();
            process_effect_damage_physical::process_effect_damage_physical(
                source_mods,
                target_mods,
                target,
                base,
            )
        }
        Effect::DamageDeal { target, amount } => {
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_damage_deal::process_effect_damage_deal(vitals, target, amount)
        }
        Effect::HealthGain { target, amount } => {
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_health_gain::process_effect_health_gain(vitals, amount)
        }
        Effect::HealthLoss { target, amount } => {
            let (vitals, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_health_loss::process_effect_health_loss(
                vitals, modifiers, target, amount,
            )
        }
        Effect::BlockGain {
            target,
            amount,
            from_card,
        } => {
            let (vitals, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_block_gain::process_effect_block_gain(
                vitals, modifiers, amount, from_card,
            )
        }
        Effect::BlockSet { target, amount } => {
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_block_set::process_effect_block_set(vitals, amount)
        }
        Effect::EnergyGain { amount } => {
            process_effect_energy_gain::process_effect_energy_gain(&mut state.energy, amount)
        }
        Effect::EnergyLoss { amount } => {
            process_effect_energy_loss::process_effect_energy_loss(&mut state.energy, amount)
        }
        Effect::ModifierGain {
            target,
            kind,
            stacks,
        } => {
            let entity = &mut state.entities[target.0 as usize];
            let monster_copy = match &entity.kind {
                EntityKind::Monster(m) => Some(*m),
                _ => None,
            };
            let (_, modifiers) = entity.kind.combatant_mut();
            process_effect_modifier_gain::process_effect_modifier_gain(
                modifiers,
                kind,
                stacks,
                monster_copy.as_ref().map(|m| m.history_slice()),
            )
        }
        Effect::ModifierRemove { target, kind } => {
            let (_, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_modifier_remove::process_effect_modifier_remove(modifiers, kind)
        }
        Effect::ModifierTick { target } => {
            let (_, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_modifier_tick::process_effect_modifier_tick(modifiers)
        }
        Effect::ModifierSetNotNew => {
            let alive = state.alive_monster_ids();
            process_effect_modifier_set_not_new::process_effect_modifier_set_not_new(
                &mut state.entities,
                &alive,
            )
        }
        Effect::Death { actor } => process_effect_death::process_effect_death(
            actor,
            &mut state.entities,
            &state.monsters,
            state.monster_count,
        ),
        Effect::CombatStart => process_effect_combat_start::process_effect_combat_start(
            &state.deck,
            &mut state.entities,
            &mut state.draw_pile,
            &mut state.hand,
            &mut state.discard_pile,
            &mut state.exhaust_pile,
            &mut state.card_active,
            &mut state.card_target,
            &state.monsters,
            state.monster_count,
            &mut state.rng,
        ),
        Effect::CombatEnd => process_effect_combat_end::process_effect_combat_end(
            &mut state.hand,
            &mut state.draw_pile,
            &mut state.discard_pile,
            &mut state.exhaust_pile,
            &mut state.card_active,
            &mut state.card_target,
            &mut state.entities,
            &mut state.monster_count,
            &state.map,
        ),
        Effect::TurnStart { actor } => {
            let monster_ids = state.alive_monster_ids();
            let (vitals, modifiers) = state.entities[actor.0 as usize].kind.combatant_mut();
            process_effect_turn_start::process_effect_turn_start(
                vitals,
                modifiers,
                actor,
                &state.energy,
                &monster_ids,
            )
        }
        Effect::TurnEnd { actor } => {
            if actor.0 == 0 {
                let alive = state.alive_monster_ids();
                process_effect_turn_end::process_effect_turn_end_character(
                    &state.entities,
                    state.card_target,
                    &alive,
                )
            } else {
                let (vitals, modifiers) = state.entities[actor.0 as usize].kind.combatant_mut();
                process_effect_turn_end::process_effect_turn_end_monster(vitals, modifiers, actor)
            }
        }
        Effect::MoveUpdate { monster } => {
            let entity = &mut state.entities[monster.0 as usize];
            process_effect_move_update::process_effect_move_update(entity, &mut state.rng)
        }
        Effect::RoomEnter => process_effect_room_enter::process_effect_room_enter(
            &state.map,
            state.ascension,
            &mut state.entities,
            &mut state.monsters,
            &mut state.monster_count,
            &mut state.rng,
        ),
        Effect::GameEnd => {
            state.effect_queue.push_front(Effect::GameEnd);
            ProcessEffectResult::Halt
        }
        Effect::AwaitMapNode => {
            state.effect_queue.push_front(Effect::AwaitMapNode);
            ProcessEffectResult::Pause
        }
        Effect::AwaitCardReward => {
            state.effect_queue.push_front(Effect::AwaitCardReward);
            ProcessEffectResult::Pause
        }
        Effect::AwaitDiscard => {
            state.effect_queue.push_front(Effect::AwaitDiscard);
            ProcessEffectResult::Pause
        }
    }
}

// ---------------------------------------------------------------------------
// Queue processing loop
// ---------------------------------------------------------------------------

pub fn process_queue(state: &mut GameState) {
    while let Some(effect) = state.effect_queue.pop_front() {
        if matches!(effect, Effect::CombatEnd) {
            state.effect_queue.clear();
        }

        let result = process_effect(state, effect);

        match result {
            ProcessEffectResult::Pass => {}
            ProcessEffectResult::Continue { top, bot } => {
                for e in top.into_iter().rev() {
                    state.effect_queue.push_front(e);
                }
                for e in bot {
                    state.effect_queue.push_back(e);
                }
            }
            ProcessEffectResult::Halt | ProcessEffectResult::Pause => return,
        }
    }
}
