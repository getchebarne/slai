pub mod process_effect_add_shivs;
pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_active_clear;
pub mod process_effect_card_active_set;
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

use crate::effect::{Effect, EffectTemplate, SelectionKind, TargetKind};
use crate::monsters::Monster;
use crate::state::{GameState, Vitals};
use crate::types::EntityId;

pub enum ProcessEffectResult {
    Continue { top: Vec<Effect>, bot: Vec<Effect> },
    Pass,
    Halt,
    Pause,
}

fn vitals_mut(state: &mut GameState, id: EntityId) -> &mut Vitals {
    if id == state.character.id {
        return &mut state.character.vitals;
    }
    &mut state.monsters
        .iter_mut()
        .find(|m| m.id == id)
        .expect("Entity not found")
        .vitals
}

fn vitals_ref(state: &GameState, id: EntityId) -> &Vitals {
    if id == state.character.id {
        return &state.character.vitals;
    }
    &state.monsters
        .iter()
        .find(|m| m.id == id)
        .expect("Entity not found")
        .vitals
}

pub fn resolve_target_kind(
    target_kind: TargetKind,
    source: EntityId,
    card_target: Option<EntityId>,
    character_id: EntityId,
    monsters: &[Monster],
) -> Vec<EntityId> {
    match target_kind {
        TargetKind::CardTarget => vec![card_target.unwrap()],
        TargetKind::Character => vec![character_id],
        TargetKind::AllMonsters => monsters.iter().map(|m| m.id).collect(),
        TargetKind::Source => vec![source],
    }
}

pub fn instantiate_templates(
    templates: &[EffectTemplate],
    source: EntityId,
    card_target: Option<EntityId>,
    character_id: EntityId,
    monsters: &[Monster],
) -> Vec<Effect> {
    let mut out = Vec::new();
    for tmpl in templates {
        match *tmpl {
            EffectTemplate::DamagePhysical { base, target } => {
                for actor in resolve_target_kind(target, source, card_target, character_id, monsters) {
                    out.push(Effect::DamagePhysical { source, target: actor, base });
                }
            }
            EffectTemplate::BlockGain { amount, target } => {
                for actor in resolve_target_kind(target, source, card_target, character_id, monsters) {
                    out.push(Effect::BlockGain { target: actor, amount, from_card: true });
                }
            }
            EffectTemplate::ModifierGain { kind, stacks, target } => {
                for actor in resolve_target_kind(target, source, card_target, character_id, monsters) {
                    out.push(Effect::ModifierGain { target: actor, kind, stacks });
                }
            }
            EffectTemplate::ModifierRemove { kind, target } => {
                for actor in resolve_target_kind(target, source, card_target, character_id, monsters) {
                    out.push(Effect::ModifierRemove { target: actor, kind });
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
            EffectTemplate::CardDiscard { selection } => match selection {
                SelectionKind::Input => {
                    out.push(Effect::AwaitDiscard);
                }
                SelectionKind::Random => {
                    out.push(Effect::CardDiscardAll);
                }
            },
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
        Effect::CardDraw { count } => {
            process_effect_card_draw::process_effect_card_draw(
                count,
                &mut state.draw_pile,
                &mut state.hand,
                &mut state.discard_pile,
                &mut state.rng,
            )
        }
        Effect::CardPlay { card_idx } => {
            process_effect_card_play::process_effect_card_play(
                card_idx,
                &state.character,
                &state.monsters,
                state.card_target,
                &state.combat_cards,
                state.character.id,
            )
        }
        Effect::CardDiscard { card_idx } => {
            process_effect_card_discard::process_effect_card_discard(
                card_idx,
                &mut state.hand,
                &mut state.discard_pile,
            )
        }
        Effect::CardDiscardAll => {
            process_effect_card_discard_all::process_effect_card_discard_all(
                &mut state.hand,
                &mut state.discard_pile,
            )
        }
        Effect::CardExhaust { card_idx } => {
            process_effect_card_exhaust::process_effect_card_exhaust(
                card_idx,
                &mut state.hand,
                &mut state.exhaust_pile,
            )
        }
        Effect::CardRemove { card_idx } => {
            process_effect_card_remove::process_effect_card_remove(
                card_idx,
                &mut state.hand,
            )
        }
        Effect::CardActiveSet { card_idx } => {
            process_effect_card_active_set::process_effect_card_active_set(
                &mut state.card_active,
                card_idx,
            )
        }
        Effect::CardActiveClear => {
            process_effect_card_active_clear::process_effect_card_active_clear(
                &mut state.card_active,
            )
        }
        Effect::AddShivs { count } => {
            process_effect_add_shivs::process_effect_add_shivs(
                count,
                &mut state.combat_cards,
                &mut state.hand,
                &mut state.discard_pile,
            )
        }
        Effect::CalculatedGamble => {
            process_effect_calculated_gamble::process_effect_calculated_gamble(
                &mut state.hand,
            )
        }
        Effect::CardUpgrade { deck_idx } => {
            process_effect_card_upgrade::process_effect_card_upgrade(
                deck_idx,
                &mut state.deck,
            )
        }
        Effect::CardRewardRoll => {
            process_effect_card_reward_roll::process_effect_card_reward_roll(
                &mut state.card_rewards,
                &mut state.character.reward_roll_offset,
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
            process_effect_target_set::process_effect_target_set(
                &mut state.card_target,
                target,
            )
        }
        Effect::TargetClear => {
            process_effect_target_clear::process_effect_target_clear(
                &mut state.card_target,
            )
        }
        Effect::DamagePhysical { source, target, base } => {
            let source_mods = &vitals_ref(state, source).modifiers;
            let target_mods = &vitals_ref(state, target).modifiers;
            process_effect_damage_physical::process_effect_damage_physical(
                source_mods,
                target_mods,
                target,
                base,
            )
        }
        Effect::DamageDeal { target, amount } => {
            let vitals = vitals_mut(state, target);
            process_effect_damage_deal::process_effect_damage_deal(vitals, target, amount)
        }
        Effect::HealthGain { target, amount } => {
            let vitals = vitals_mut(state, target);
            process_effect_health_gain::process_effect_health_gain(vitals, amount)
        }
        Effect::HealthLoss { target, amount } => {
            let character_id = state.character.id;
            let vitals = vitals_mut(state, target);
            process_effect_health_loss::process_effect_health_loss(vitals, target, amount, character_id)
        }
        Effect::BlockGain { target, amount, from_card } => {
            let vitals = vitals_mut(state, target);
            process_effect_block_gain::process_effect_block_gain(vitals, amount, from_card)
        }
        Effect::BlockSet { target, amount } => {
            let vitals = vitals_mut(state, target);
            process_effect_block_set::process_effect_block_set(vitals, amount)
        }
        Effect::EnergyGain { amount } => {
            process_effect_energy_gain::process_effect_energy_gain(
                &mut state.energy,
                amount,
            )
        }
        Effect::EnergyLoss { amount } => {
            process_effect_energy_loss::process_effect_energy_loss(
                &mut state.energy,
                amount,
            )
        }
        Effect::ModifierGain { target, kind, stacks } => {
            if let Some(monster) = state.monsters.iter_mut().find(|m| m.id == target) {
                process_effect_modifier_gain::process_effect_modifier_gain(
                    &mut monster.vitals.modifiers,
                    kind,
                    stacks,
                    Some(&monster.move_history),
                )
            } else {
                process_effect_modifier_gain::process_effect_modifier_gain(
                    &mut state.character.vitals.modifiers,
                    kind,
                    stacks,
                    None,
                )
            }
        }
        Effect::ModifierRemove { target, kind } => {
            let modifiers = &mut vitals_mut(state, target).modifiers;
            process_effect_modifier_remove::process_effect_modifier_remove(modifiers, kind)
        }
        Effect::ModifierTick { target } => {
            let modifiers = &mut vitals_mut(state, target).modifiers;
            process_effect_modifier_tick::process_effect_modifier_tick(modifiers)
        }
        Effect::ModifierSetNotNew => {
            process_effect_modifier_set_not_new::process_effect_modifier_set_not_new(
                &mut state.character,
                &mut state.monsters,
            )
        }
        Effect::Death { actor } => {
            process_effect_death::process_effect_death(actor, &mut state.monsters, state.character.id)
        }
        Effect::CombatStart => {
            let monster_ids: Vec<EntityId> = state.monsters.iter().map(|m| m.id).collect();
            process_effect_combat_start::process_effect_combat_start(
                &state.deck,
                &mut state.combat_cards,
                &mut state.draw_pile,
                &mut state.hand,
                &mut state.discard_pile,
                &mut state.exhaust_pile,
                &mut state.card_active,
                &mut state.card_target,
                &monster_ids,
                state.character.id,
                &mut state.rng,
            )
        }
        Effect::CombatEnd => {
            process_effect_combat_end::process_effect_combat_end(
                &mut state.hand,
                &mut state.draw_pile,
                &mut state.discard_pile,
                &mut state.exhaust_pile,
                &mut state.combat_cards,
                &mut state.card_active,
                &mut state.card_target,
                &mut state.character.vitals.modifiers,
                &state.map,
            )
        }
        Effect::TurnStart { actor } => {
            let character_id = state.character.id;
            let monster_ids: Vec<EntityId> = state.monsters.iter().map(|m| m.id).collect();
            if actor == character_id {
                process_effect_turn_start::process_effect_turn_start(
                    &mut state.character.vitals,
                    actor,
                    &state.energy,
                    &monster_ids,
                    character_id,
                )
            } else {
                let monster = state.monsters.iter_mut().find(|m| m.id == actor)
                    .expect("Monster not found for TurnStart");
                process_effect_turn_start::process_effect_turn_start(
                    &mut monster.vitals,
                    actor,
                    &state.energy,
                    &monster_ids,
                    character_id,
                )
            }
        }
        Effect::TurnEnd { actor } => {
            if actor == state.character.id {
                process_effect_turn_end::process_effect_turn_end_character(
                    &mut state.character.vitals,
                    &state.monsters,
                    state.card_target,
                    state.character.id,
                )
            } else {
                let monster = state.monsters.iter_mut().find(|m| m.id == actor)
                    .expect("Monster not found for TurnEnd");
                process_effect_turn_end::process_effect_turn_end_monster(
                    &mut monster.vitals,
                    actor,
                )
            }
        }
        Effect::MoveUpdate { monster } => {
            let m = state.monsters.iter_mut().find(|m| m.id == monster)
                .expect("Monster not found for MoveUpdate");
            process_effect_move_update::process_effect_move_update(m, &mut state.rng)
        }
        Effect::RoomEnter => {
            process_effect_room_enter::process_effect_room_enter(
                &state.map,
                state.ascension,
                &mut state.monsters,
                &mut state.next_entity_id,
                &mut state.rng,
            )
        }
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
