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
pub mod process_effect_rest_site_exit;
pub mod process_effect_room_enter;
pub mod process_effect_target_clear;
pub mod process_effect_target_set;
pub mod process_effect_turn_end;
pub mod process_effect_turn_start;

use rand::Rng;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH};
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::state::{Entity, EntityKind, GameState, Map};
use crate::types::{EntityId, Phase};
use crate::utils::{get_alive_monster_ids, shuffle};

pub enum ProcessEffectResult {
    AddAndContinue {
        top: Vec<Effect>,
        bot: Vec<Effect>,
    },
    Continue,
    Replace(Vec<Effect>),
    /// The queue driver sets `state.phase = phase_new` and returns.
    Halt {
        phase_new: Phase,
    },
}

pub enum TargetResolution {
    Resolved(Vec<EntityId>),
    AwaitInput { num: u8 },
}

pub(crate) fn resolve_candidates(
    candidates: CandidatePool,
    source: EntityId,
    character: EntityId,
    hand: &[EntityId],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
    map: &Map,
    entities: &[Entity],
    card_rewards: &[EntityId],
) -> Vec<EntityId> {
    match candidates {
        CandidatePool::Hand => hand.to_vec(),
        CandidatePool::CardTarget => vec![card_target.unwrap()],
        CandidatePool::Character => vec![character],
        CandidatePool::Monsters => alive_monsters.to_vec(),
        CandidatePool::Source => vec![source],
        CandidatePool::CardRewardPool => card_rewards.to_vec(),
        CandidatePool::MapNodeNextRow => {
            let y_next = match map.y_current {
                None => 0,
                Some(y) => y + 1,
            };
            if y_next >= MAP_HEIGHT {
                return Vec::new();
            }
            let mut out = Vec::new();
            if let Some(y) = map.y_current {
                let x = map.x_current.unwrap();
                if let Some(current_id) = map.nodes[y][x] {
                    let current_node = entities[current_id.0 as usize].kind.map_node_ref();
                    for col in 0..MAP_WIDTH {
                        if current_node.has_edge(col) {
                            if let Some(id) = map.nodes[y_next][col] {
                                out.push(id);
                            }
                        }
                    }
                }
            } else {
                for col in 0..MAP_WIDTH {
                    if let Some(id) = map.nodes[0][col] {
                        out.push(id);
                    }
                }
            }
            out
        }
    }
}

fn resolve_targets(
    candidates: CandidatePool,
    selection: SelectionKind,
    source: EntityId,
    character: EntityId,
    hand: &[EntityId],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
    map: &Map,
    entities: &[Entity],
    card_rewards: &[EntityId],
    rng: &mut impl Rng,
) -> TargetResolution {
    let mut ids = resolve_candidates(
        candidates,
        source,
        character,
        hand,
        card_target,
        alive_monsters,
        map,
        entities,
        card_rewards,
    );
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
                TargetResolution::AwaitInput { num: count }
            }
        }
    }
}

// Dispatcher entry point. Branches on `Target`:
//  - `Direct(t)` runs the kind-specific handler with an already-known target.
//  - `Resolve { .. }` runs the resolver; on success, fans out to `Direct`
//    effects; on input-needed, returns `Halt` (the effect stays at the front
//    because the driver uses peek-before-pop and won't have popped it yet).
pub fn process_effect(state: &mut GameState, effect: Effect) -> ProcessEffectResult {
    let target = match effect.target {
        Target::Direct(t) => t,
        Target::Resolve {
            candidates,
            selection,
        } => {
            return resolve_or_halt(state, effect.kind, effect.source, candidates, selection);
        }
    };

    dispatch_by_kind(state, effect.kind, effect.source, target)
}

fn resolve_or_halt(
    state: &mut GameState,
    kind: EffectKind,
    source: Option<EntityId>,
    candidates: CandidatePool,
    selection: SelectionKind,
) -> ProcessEffectResult {
    let alive = get_alive_monster_ids(state);
    let src_id = source.unwrap_or(state.character);
    let resolution = resolve_targets(
        candidates,
        selection,
        src_id,
        state.character,
        &state.hand,
        state.card_target,
        &alive,
        &state.map,
        &state.entities,
        &state.card_rewards,
        &mut state.rng,
    );
    match resolution {
        TargetResolution::Resolved(ids) => {
            let fanout: Vec<Effect> = ids
                .into_iter()
                .map(|id| Effect {
                    kind,
                    source,
                    target: Target::Direct(Some(id)),
                })
                .collect();
            ProcessEffectResult::AddAndContinue {
                top: fanout,
                bot: Vec::new(),
            }
        }
        TargetResolution::AwaitInput { num } => match kind {
            EffectKind::CardDiscard => ProcessEffectResult::Halt {
                phase_new: Phase::CombatAwaitDiscard { num },
            },
            EffectKind::MapNodeSelect => ProcessEffectResult::Halt {
                phase_new: Phase::Map,
            },
            _ => panic!("Unsupported effect kind for halting: {:?}", kind),
        },
    }
}

fn dispatch_by_kind(
    state: &mut GameState,
    kind: EffectKind,
    source: Option<EntityId>,
    target: Option<EntityId>,
) -> ProcessEffectResult {
    match kind {
        EffectKind::CardDraw { count } => process_effect_card_draw::process_effect_card_draw(
            count,
            &mut state.draw_pile,
            &mut state.hand,
            &mut state.discard_pile,
            &mut state.rng,
        ),
        EffectKind::CardPlay => {
            let id_card = target.unwrap();
            let alive = get_alive_monster_ids(state);
            process_effect_card_play::process_effect_card_play(
                id_card,
                state.card_target,
                state.character,
                &state.entities,
                &state.hand,
                &alive,
                &mut state.rng,
            )
        }
        EffectKind::CardDiscard => {
            let id_card = target.unwrap();
            process_effect_card_discard::process_effect_card_discard(
                id_card,
                &mut state.hand,
                &mut state.discard_pile,
            )
        }
        EffectKind::CardExhaust => {
            let id_card = target.unwrap();
            process_effect_card_exhaust::process_effect_card_exhaust(
                id_card,
                &mut state.hand,
                &mut state.exhaust_pile,
            )
        }
        EffectKind::CardRemove => {
            let id_card = target.unwrap();
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
        EffectKind::CardUpgrade => {
            let id_card = target.unwrap();
            process_effect_card_upgrade::process_effect_card_upgrade(id_card, &mut state.entities)
        }
        EffectKind::CardRewardRoll => {
            process_effect_card_reward_roll::process_effect_card_reward_roll(
                state.character,
                &mut state.card_rewards,
                &mut state.entities,
                &mut state.rng,
            )
        }
        EffectKind::CardRewardClear => {
            process_effect_card_reward_clear::process_effect_card_reward_clear(
                &mut state.card_rewards,
            )
        }
        EffectKind::TargetSet => {
            let target = target.unwrap();
            process_effect_target_set::process_effect_target_set(&mut state.card_target, target)
        }
        EffectKind::TargetClear => {
            process_effect_target_clear::process_effect_target_clear(&mut state.card_target)
        }
        EffectKind::DamagePhysical { base } => {
            let source = source.unwrap();
            let target = target.unwrap();
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
            let target = target.unwrap();
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_damage_deal::process_effect_damage_deal(vitals, target, amount)
        }
        EffectKind::HealthGain { amount } => {
            let target = target.unwrap();
            let (vitals, _) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_health_gain::process_effect_health_gain(vitals, amount)
        }
        EffectKind::HealthLoss { amount } => {
            let target = target.unwrap();
            let (vitals, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_health_loss::process_effect_health_loss(
                vitals,
                modifiers,
                target,
                state.character,
                amount,
            )
        }
        EffectKind::BlockGain { amount } => {
            let target = target.unwrap();
            let from_card = source
                .map(|id| matches!(state.entities[id.0 as usize].kind, EntityKind::Card(_)))
                .unwrap_or(false);
            let (vitals, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_block_gain::process_effect_block_gain(
                vitals, modifiers, amount, from_card,
            )
        }
        EffectKind::BlockSet { amount } => {
            let target = target.unwrap();
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
            let target = target.unwrap();
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
            let target = target.unwrap();
            let (_, modifiers) = state.entities[target.0 as usize].kind.combatant_mut();
            process_effect_modifier_remove::process_effect_modifier_remove(modifiers, kind)
        }
        EffectKind::ModifierTick => {
            let target = target.unwrap();
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
            let actor = target.unwrap();
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
            let actor = target.unwrap();
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
            let actor = target.unwrap();
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
            let monster = target.unwrap();
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
        EffectKind::RestSiteExit => {
            process_effect_rest_site_exit::process_effect_rest_site_exit(&mut state.map)
        }
        // Halt-kind variants: represent pending player decisions.
        // MapNodeSelect and CardRewardSelect in their `Direct` form (after
        // the resolver picked a target) complete the transition. Before
        // resolution they're handled by the `Resolve` branch in `process_effect`.
        EffectKind::MapNodeSelect => {
            let node_id = target.expect("MapNodeSelect Direct form must have target");
            let node = *state.entities[node_id.0 as usize].kind.map_node_ref();
            state.map.y_current = Some(node.y);
            state.map.x_current = Some(node.x);
            ProcessEffectResult::AddAndContinue {
                top: vec![Effect::direct(EffectKind::RoomEnter, None, None)],
                bot: Vec::new(),
            }
        }
        EffectKind::CardRewardSelect => {
            let card_id = target.expect("CardRewardSelect Direct form must have target");
            state.deck.push(card_id);
            ProcessEffectResult::AddAndContinue {
                top: vec![Effect::direct(EffectKind::CardRewardClear, None, None)],
                bot: Vec::new(),
            }
        }
        EffectKind::GameOver => ProcessEffectResult::Halt {
            phase_new: Phase::GameOver,
        },
        EffectKind::AwaitCombatAction => ProcessEffectResult::Halt {
            phase_new: Phase::CombatDefault,
        },
        EffectKind::AwaitRestSiteAction => ProcessEffectResult::Halt {
            phase_new: Phase::RestSite,
        },
        EffectKind::AwaitCardReward => ProcessEffectResult::Halt {
            phase_new: Phase::CombatReward,
        },
    }
}

pub fn process_queue(state: &mut GameState) {
    loop {
        let Some(effect) = state.effect_queue.pop_front() else {
            panic!("process_queue: queue drained without halting");
        };
        match process_effect(state, effect) {
            ProcessEffectResult::Continue => {}
            ProcessEffectResult::AddAndContinue { top, bot } => {
                for e in top.into_iter().rev() {
                    state.effect_queue.push_front(e);
                }
                for e in bot {
                    state.effect_queue.push_back(e);
                }
            }
            ProcessEffectResult::Replace(effects) => {
                state.effect_queue.clear();
                for e in effects {
                    state.effect_queue.push_back(e);
                }
            }
            ProcessEffectResult::Halt { phase_new } => {
                state.phase = phase_new;
                return;
            }
        }
    }
}
