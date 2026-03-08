pub mod process_effect_add_shivs;
pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_discard;
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

use rand::Rng;

use crate::effect::{Candidates, Effect, EffectKind, EffectTemplate, SelectionKind};
use crate::state::{EntityKind, GameState};
use crate::types::EntityId;
use crate::utils::{get_alive_monster_ids, shuffle};

pub enum ProcessEffectResult {
    AddAndContinue { top: Vec<Effect>, bot: Vec<Effect> },
    Continue,
    Replace(Vec<Effect>),
    Pause,
}

pub enum TargetResolution {
    Resolved(Vec<EntityId>),
    AwaitInput,
}

fn resolve_candidates(
    candidates: Candidates,
    source: EntityId,
    character: EntityId,
    hand: &[EntityId],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
) -> Vec<EntityId> {
    match candidates {
        Candidates::Hand => hand.to_vec(),
        Candidates::CardTarget => vec![card_target.unwrap()],
        Candidates::Character => vec![character],
        Candidates::Monsters => alive_monsters.to_vec(),
        Candidates::Source => vec![source],
    }
}

fn resolve_targets(
    candidates: Candidates,
    selection: SelectionKind,
    source: EntityId,
    character: EntityId,
    hand: &[EntityId],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
    rng: &mut impl Rng,
) -> TargetResolution {
    let mut ids = resolve_candidates(candidates, source, character, hand, card_target, alive_monsters);
    match selection {
        SelectionKind::All => TargetResolution::Resolved(ids),
        SelectionKind::Random { count } => {
            shuffle(&mut ids, rng);
            ids.truncate(count as usize);
            TargetResolution::Resolved(ids)
        }
        SelectionKind::Input { count } => {
            if count as usize >= ids.len() {
                TargetResolution::Resolved(ids)
            } else {
                TargetResolution::AwaitInput
            }
        }
    }
}

pub fn instantiate_templates(
    templates: &[EffectTemplate],
    source: EntityId,
    character: EntityId,
    hand: &[EntityId],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
    rng: &mut impl Rng,
) -> Vec<Effect> {
    let mut out = Vec::new();
    for tmpl in templates {
        match tmpl.targeting {
            None => {
                out.push(Effect {
                    kind: tmpl.kind,
                    source: None,
                    target: None,
                });
            }
            Some(targeting) => {
                let resolution = resolve_targets(
                    targeting.candidates,
                    targeting.selection,
                    source,
                    character,
                    hand,
                    card_target,
                    alive_monsters,
                    rng,
                );
                match resolution {
                    TargetResolution::Resolved(ids) => {
                        for t in ids {
                            out.push(Effect {
                                kind: tmpl.kind,
                                source: Some(source),
                                target: Some(t),
                            });
                        }
                    }
                    TargetResolution::AwaitInput => {
                        // TODO: handle await input
                    }
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

pub fn process_effect(state: &mut GameState, effect: Effect) -> ProcessEffectResult {
    match effect.kind {
        EffectKind::CardDraw { count } => process_effect_card_draw::process_effect_card_draw(
            count,
            &mut state.draw_pile,
            &mut state.hand,
            &mut state.discard_pile,
            &mut state.rng,
        ),
        EffectKind::CardPlay => {
            let id_card = effect.target.unwrap();
            let alive = get_alive_monster_ids(state);
            process_effect_card_play::process_effect_card_play(
                id_card,
                state.card_target,
                state.character,
                &state.entities,
                &alive,
                &mut state.rng,
            )
        }
        EffectKind::CardDiscard => {
            let id_card = effect.target.unwrap();
            process_effect_card_discard::process_effect_card_discard(
                id_card,
                &mut state.hand,
                &mut state.discard_pile,
            )
        }
        EffectKind::CardExhaust => {
            let id_card = effect.target.unwrap();
            process_effect_card_exhaust::process_effect_card_exhaust(
                id_card,
                &mut state.hand,
                &mut state.exhaust_pile,
            )
        }
        EffectKind::CardRemove => {
            let id_card = effect.target.unwrap();
            process_effect_card_remove::process_effect_card_remove(id_card, &mut state.hand)
        }
        EffectKind::AddShivs { count } => process_effect_add_shivs::process_effect_add_shivs(
            count,
            &mut state.entities,
            &mut state.hand,
            &mut state.discard_pile,
        ),
        EffectKind::CalculatedGamble => {
            process_effect_calculated_gamble::process_effect_calculated_gamble(&state.hand)
        }
        EffectKind::CardUpgrade { deck_idx } => {
            process_effect_card_upgrade::process_effect_card_upgrade(
                deck_idx,
                &state.deck,
                &mut state.entities,
            )
        }
        EffectKind::CardRewardRoll => {
            process_effect_card_reward_roll::process_effect_card_reward_roll(
                state.character,
                &mut state.card_rewards,
                &mut state.entities,
                &mut state.rng,
            )
        }
        EffectKind::CardRewardSelect { idx_reward } => {
            process_effect_card_reward_select::process_effect_card_reward_select(
                idx_reward,
                &mut state.card_rewards,
                &mut state.deck,
            )
        }
        EffectKind::CardRewardClear => {
            process_effect_card_reward_clear::process_effect_card_reward_clear(
                &mut state.card_rewards,
            )
        }
        EffectKind::TargetSet => {
            let target = effect.target.unwrap();
            process_effect_target_set::process_effect_target_set(&mut state.card_target, target)
        }
        EffectKind::TargetClear => {
            process_effect_target_clear::process_effect_target_clear(&mut state.card_target)
        }
        EffectKind::DamagePhysical { base } => {
            let source = effect.source.unwrap();
            let target = effect.target.unwrap();
            let (_, source_mods) = state.entities[source.0 as usize].kind.combatant_ref();
            let (_, target_mods) = state.entities[target.0 as usize].kind.combatant_ref();
            process_effect_damage_physical::process_effect_damage_physical(
                source_mods,
                target_mods,
                target,
                base,
            )
        }
        EffectKind::DamageDeal { amount } => {
            let target = effect.target.unwrap();
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_damage_deal::process_effect_damage_deal(vitals, target, amount)
        }
        EffectKind::HealthGain { amount } => {
            let target = effect.target.unwrap();
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_health_gain::process_effect_health_gain(vitals, amount)
        }
        EffectKind::HealthLoss { amount } => {
            let target = effect.target.unwrap();
            let (vitals, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_health_loss::process_effect_health_loss(
                vitals, modifiers, target, amount,
            )
        }
        EffectKind::BlockGain { amount } => {
            let target = effect.target.unwrap();
            let from_card = effect
                .source
                .map(|id| matches!(state.entities[id.0 as usize].kind, EntityKind::Card(_)))
                .unwrap_or(false);
            let (vitals, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_block_gain::process_effect_block_gain(
                vitals, modifiers, amount, from_card,
            )
        }
        EffectKind::BlockSet { amount } => {
            let target = effect.target.unwrap();
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_block_set::process_effect_block_set(vitals, amount)
        }
        EffectKind::EnergyGain { amount } => {
            process_effect_energy_gain::process_effect_energy_gain(&mut state.energy, amount)
        }
        EffectKind::EnergyLoss { amount } => {
            process_effect_energy_loss::process_effect_energy_loss(&mut state.energy, amount)
        }
        EffectKind::ModifierGain { kind, stacks } => {
            let target = effect.target.unwrap();
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
        EffectKind::ModifierRemove { kind } => {
            let target = effect.target.unwrap();
            let (_, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_modifier_remove::process_effect_modifier_remove(modifiers, kind)
        }
        EffectKind::ModifierTick => {
            let target = effect.target.unwrap();
            let (_, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_modifier_tick::process_effect_modifier_tick(modifiers)
        }
        EffectKind::ModifierSetNotNew => {
            let alive = get_alive_monster_ids(state);
            process_effect_modifier_set_not_new::process_effect_modifier_set_not_new(
                state.character,
                &mut state.entities,
                &alive,
            )
        }
        EffectKind::Death => {
            let actor = effect.target.unwrap();
            process_effect_death::process_effect_death(
                actor,
                state.character,
                &mut state.entities,
                &state.monsters,
                state.monster_count,
            )
        }
        EffectKind::CombatStart => process_effect_combat_start::process_effect_combat_start(
            state.character,
            &state.deck,
            &mut state.entities,
            &mut state.draw_pile,
            &mut state.hand,
            &mut state.discard_pile,
            &mut state.exhaust_pile,
            &mut state.card_target,
            &state.monsters,
            state.monster_count,
            &mut state.rng,
        ),
        EffectKind::CombatEnd => process_effect_combat_end::process_effect_combat_end(
            state.character,
            &mut state.hand,
            &mut state.draw_pile,
            &mut state.discard_pile,
            &mut state.exhaust_pile,
            &mut state.card_target,
            &mut state.entities,
            &mut state.monster_count,
            &state.map,
        ),
        EffectKind::TurnStart => {
            let actor = effect.target.unwrap();
            let monster_ids = get_alive_monster_ids(state);
            let (vitals, modifiers) = state.entities[actor.0 as usize].kind.combatant_mut();
            process_effect_turn_start::process_effect_turn_start(
                vitals,
                modifiers,
                actor,
                state.character,
                &state.energy,
                &monster_ids,
            )
        }
        EffectKind::TurnEnd => {
            let actor = effect.target.unwrap();
            if actor == state.character {
                let alive = get_alive_monster_ids(state);
                process_effect_turn_end::process_effect_turn_end_character(
                    state.character,
                    &state.entities,
                    &state.hand,
                    state.card_target,
                    &alive,
                    &mut state.rng,
                )
            } else {
                let (vitals, modifiers) = state.entities[actor.0 as usize].kind.combatant_mut();
                process_effect_turn_end::process_effect_turn_end_monster(vitals, modifiers, actor)
            }
        }
        EffectKind::MoveUpdate => {
            let monster = effect.target.unwrap();
            let entity = &mut state.entities[monster.0 as usize];
            process_effect_move_update::process_effect_move_update(entity, &mut state.rng)
        }
        EffectKind::RoomEnter => process_effect_room_enter::process_effect_room_enter(
            &state.map,
            state.ascension,
            &mut state.entities,
            &mut state.monsters,
            &mut state.monster_count,
            &mut state.rng,
        ),
        EffectKind::GameEnd => {
            state.effect_queue.push_front(effect);
            ProcessEffectResult::Pause
        }
        EffectKind::AwaitMapNode => {
            state.effect_queue.push_front(effect);
            ProcessEffectResult::Pause
        }
        EffectKind::AwaitCardReward => {
            state.effect_queue.push_front(effect);
            ProcessEffectResult::Pause
        }
        EffectKind::AwaitDiscard => {
            state.effect_queue.push_front(effect);
            ProcessEffectResult::Pause
        }
    }
}

// ---------------------------------------------------------------------------
// Queue processing loop
// ---------------------------------------------------------------------------

pub fn process_queue(state: &mut GameState) {
    while let Some(effect) = state.effect_queue.pop_front() {
        // Process the effect
        let result = process_effect(state, effect);

        // Gate based on the processing result
        match result {
            // Continue to next effect immediately
            ProcessEffectResult::Continue => {}

            // Prepend and/or append and continue to next effect
            ProcessEffectResult::AddAndContinue { top, bot } => {
                for e in top.into_iter().rev() {
                    state.effect_queue.push_front(e);
                }
                for e in bot {
                    state.effect_queue.push_back(e);
                }
            }

            // Replace queue w/ new one
            ProcessEffectResult::Replace(effects) => {
                state.effect_queue.clear();
                for e in effects {
                    state.effect_queue.push_back(e);
                }
            }

            // Return, potentially leaving unprocessed effects
            ProcessEffectResult::Pause => return,
        }
    }
}
