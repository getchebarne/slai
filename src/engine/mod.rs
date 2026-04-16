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

use std::collections::VecDeque;

use rand::Rng;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH, MAX_MONSTERS};
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, EntityType};
use crate::map::has_edge;
use crate::state::{GameState, Map, Position};
use crate::types::Phase;
use crate::utils::{fill_alive_monster_ids, shuffle};

pub enum DispatchResult {
    Continue,
    /// The queue driver sets `state.phase = phase_new` and returns.
    Halt {
        phase_new: Phase,
    },
}

// Stack-allocated buffer for effects that a handler wants to push to the
// front of the queue in order. Build up effects normally, then call
// `push_all_front` once at the end of the handler.
pub const MAX_EFFECTS_PER_HANDLER: usize = 32;

const ZERO_EFFECT: Effect = Effect {
    kind: EffectKind::CardDiscard,
    source: None,
    target: Target::Direct(None),
};

pub struct EffectBuf {
    pub effects: [Effect; MAX_EFFECTS_PER_HANDLER],
    pub len: usize,
}

impl EffectBuf {
    pub const fn new() -> Self {
        Self {
            effects: [ZERO_EFFECT; MAX_EFFECTS_PER_HANDLER],
            len: 0,
        }
    }

    pub fn push(&mut self, e: Effect) {
        assert!(self.len < MAX_EFFECTS_PER_HANDLER, "EffectBuf overflow");
        self.effects[self.len] = e;
        self.len += 1;
    }

    pub fn push_all_front(&self, queue: &mut VecDeque<Effect>) {
        for e in self.effects[..self.len].iter().rev() {
            queue.push_front(*e);
        }
    }
}

// Stack-allocated buffer for candidate ids during resolution. All candidate
// pools have compile-time-bounded sizes (≤ MAX_SIZE_HAND, MAX_MONSTERS, etc.),
// so we can avoid the heap entirely.
pub const MAX_CANDIDATES: usize = 16;

pub struct CandidateBuf {
    pub ids: [usize; MAX_CANDIDATES],
    pub len: usize,
}

impl CandidateBuf {
    pub fn new() -> Self {
        Self {
            ids: [0; MAX_CANDIDATES],
            len: 0,
        }
    }

    pub fn push(&mut self, id: usize) {
        assert!(self.len < MAX_CANDIDATES, "CandidateBuf overflow");
        self.ids[self.len] = id;
        self.len += 1;
    }

    pub fn as_slice(&self) -> &[usize] {
        &self.ids[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [usize] {
        &mut self.ids[..self.len]
    }

    pub fn truncate(&mut self, n: usize) {
        if n < self.len {
            self.len = n;
        }
    }

    pub fn extend_from_slice(&mut self, src: &[usize]) {
        for &id in src {
            self.push(id);
        }
    }
}

pub enum TargetResolution {
    Resolved,
    AwaitInput { num: u8 },
}

pub(crate) fn resolve_candidates(
    candidates: CandidatePool,
    source: usize,
    character: usize,
    hand: &[usize],
    card_target: Option<usize>,
    alive_monsters: &[usize],
    map: &Map,
    entities: &[Entity],
    card_rewards: &[usize],
    out: &mut CandidateBuf,
) {
    match candidates {
        CandidatePool::Hand => out.extend_from_slice(hand),
        CandidatePool::CardTarget => out.push(card_target.unwrap()),
        CandidatePool::Character => out.push(character),
        CandidatePool::Monsters => out.extend_from_slice(alive_monsters),
        CandidatePool::Source => out.push(source),
        CandidatePool::CardRewardPool => out.extend_from_slice(card_rewards),
        CandidatePool::NextRowRooms => match map.position {
            Position::Start => {
                for col in 0..MAP_WIDTH {
                    if let Some(id) = map.nodes[0][col] {
                        out.push(id);
                    }
                }
            }
            Position::Overworld { y, x } => {
                let y_next = y + 1;
                if y_next >= MAP_HEIGHT {
                    return;
                }
                if let Some(current_id) = map.nodes[y][x] {
                    let current_node = &entities[current_id];
                    for col in 0..MAP_WIDTH {
                        if has_edge(current_node.edges, col) {
                            if let Some(id) = map.nodes[y_next][col] {
                                out.push(id);
                            }
                        }
                    }
                }
            }
            Position::BossRoom => {}
        },
    }
}

fn resolve_targets(
    candidates: CandidatePool,
    selection: SelectionKind,
    source: usize,
    character: usize,
    hand: &[usize],
    card_target: Option<usize>,
    alive_monsters: &[usize],
    map: &Map,
    entities: &[Entity],
    card_rewards: &[usize],
    rng: &mut impl Rng,
    out: &mut CandidateBuf,
) -> TargetResolution {
    resolve_candidates(
        candidates,
        source,
        character,
        hand,
        card_target,
        alive_monsters,
        map,
        entities,
        card_rewards,
        out,
    );
    match selection {
        SelectionKind::All => TargetResolution::Resolved,
        SelectionKind::Random { count } => {
            shuffle(out.as_mut_slice(), rng);
            out.truncate(count as usize);
            TargetResolution::Resolved
        }
        SelectionKind::Input { count } => {
            if count as usize >= out.len {
                TargetResolution::Resolved
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
pub fn process_effect(state: &mut GameState, effect: Effect) -> DispatchResult {
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
    source: Option<usize>,
    candidates: CandidatePool,
    selection: SelectionKind,
) -> DispatchResult {
    let mut alive_buf = [0usize; MAX_MONSTERS];
    let alive_n = fill_alive_monster_ids(state, &mut alive_buf);
    let src_id = source.unwrap_or(state.character);
    let mut cands = CandidateBuf::new();
    let resolution = resolve_targets(
        candidates,
        selection,
        src_id,
        state.character,
        &state.hand,
        state.card_target,
        &alive_buf[..alive_n],
        &state.map,
        &state.entities,
        &state.card_rewards,
        &mut state.rng,
        &mut cands,
    );
    match resolution {
        TargetResolution::Resolved => {
            // Push one Direct-target effect per resolved id. push_front reverses
            // order, so iterate in reverse to preserve id order in the queue.
            for &id in cands.as_slice().iter().rev() {
                state.effect_queue.push_front(Effect {
                    kind,
                    source,
                    target: Target::Direct(Some(id)),
                });
            }
            DispatchResult::Continue
        }
        TargetResolution::AwaitInput { num } => match kind {
            EffectKind::CardDiscard => DispatchResult::Halt {
                phase_new: Phase::CombatAwaitDiscard { num },
            },
            EffectKind::RoomSelect => DispatchResult::Halt {
                phase_new: Phase::Map,
            },
            _ => panic!("Unsupported effect kind for halting: {:?}", kind),
        },
    }
}

fn dispatch_by_kind(
    state: &mut GameState,
    kind: EffectKind,
    source: Option<usize>,
    target: Option<usize>,
) -> DispatchResult {
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
            let mut alive_buf = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut alive_buf);
            process_effect_card_play::process_effect_card_play(
                id_card,
                state.card_target,
                state.character,
                &state.entities,
                &state.hand,
                &alive_buf[..alive_n],
                &mut state.rng,
                &mut state.effect_queue,
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
            process_effect_calculated_gamble::process_effect_calculated_gamble(
                &state.hand,
                &mut state.effect_queue,
            )
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
                &mut state.effect_queue,
            )
        }
        EffectKind::CardRewardClear => {
            process_effect_card_reward_clear::process_effect_card_reward_clear(
                &mut state.card_rewards,
                &mut state.effect_queue,
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
            let source_mods = &state.entities[source].modifiers;
            let target_mods = &state.entities[target].modifiers;
            process_effect_damage_physical::process_effect_damage_physical(
                source_mods,
                target_mods,
                target,
                base,
                &mut state.effect_queue,
            )
        }
        EffectKind::DamageDeal { amount } => {
            let target = target.unwrap();
            let vitals = &mut state.entities[target].vitals;
            process_effect_damage_deal::process_effect_damage_deal(
                vitals,
                target,
                amount,
                &mut state.effect_queue,
            )
        }
        EffectKind::HealthGain { amount } => {
            let target = target.unwrap();
            let vitals = &mut state.entities[target].vitals;
            process_effect_health_gain::process_effect_health_gain(vitals, amount)
        }
        EffectKind::HealthLoss { amount } => {
            let target = target.unwrap();
            let entity = &mut state.entities[target];
            process_effect_health_loss::process_effect_health_loss(
                &mut entity.vitals,
                &mut entity.modifiers,
                target,
                state.character,
                amount,
                &mut state.effect_queue,
            )
        }
        EffectKind::BlockGain { amount } => {
            let target = target.unwrap();
            let from_card = source
                .map(|id| state.entities[id].kind == EntityType::Card)
                .unwrap_or(false);
            let entity = &mut state.entities[target];
            process_effect_block_gain::process_effect_block_gain(
                &mut entity.vitals,
                &mut entity.modifiers,
                amount,
                from_card,
            )
        }
        EffectKind::BlockSet { amount } => {
            let target = target.unwrap();
            let vitals = &mut state.entities[target].vitals;
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
            let entity = &mut state.entities[target];
            let cycle_count = if entity.kind == EntityType::Monster {
                Some(entity.cycle_count)
            } else {
                None
            };
            process_effect_modifier_gain::process_effect_modifier_gain(
                &mut entity.modifiers,
                kind,
                stacks,
                cycle_count,
            )
        }
        EffectKind::ModifierRemove { kind } => {
            let target = target.unwrap();
            let modifiers = &mut state.entities[target].modifiers;
            process_effect_modifier_remove::process_effect_modifier_remove(modifiers, kind)
        }
        EffectKind::ModifierTick => {
            let target = target.unwrap();
            let modifiers = &mut state.entities[target].modifiers;
            process_effect_modifier_tick::process_effect_modifier_tick(modifiers)
        }
        EffectKind::ModifierSetNotNew => {
            let mut alive_buf = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut alive_buf);
            process_effect_modifier_set_not_new::process_effect_modifier_set_not_new(
                state.character,
                &mut state.entities,
                &alive_buf[..alive_n],
            )
        }
        EffectKind::Death => {
            let actor = target.unwrap();
            process_effect_death::process_effect_death(
                actor,
                state.character,
                &state.monsters,
                state.monster_count,
                &mut state.entities,
                &mut state.effect_queue,
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
            &mut state.effect_queue,
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
            &mut state.effect_queue,
        ),
        EffectKind::TurnStart => {
            let actor = target.unwrap();
            let mut alive_buf = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut alive_buf);
            let entity = &mut state.entities[actor];
            process_effect_turn_start::process_effect_turn_start(
                &mut entity.vitals,
                &mut entity.modifiers,
                actor,
                state.character,
                &state.energy,
                &alive_buf[..alive_n],
                &mut state.effect_queue,
            )
        }
        EffectKind::TurnEnd => {
            let actor = target.unwrap();
            if actor == state.character {
                let mut alive_buf = [0usize; MAX_MONSTERS];
                let alive_n = fill_alive_monster_ids(state, &mut alive_buf);
                process_effect_turn_end::process_effect_turn_end_character(
                    state.character,
                    &state.entities,
                    &state.hand,
                    state.card_target,
                    &alive_buf[..alive_n],
                    &mut state.rng,
                    &mut state.effect_queue,
                )
            } else {
                let entity = &mut state.entities[actor];
                process_effect_turn_end::process_effect_turn_end_monster(
                    &mut entity.vitals,
                    &mut entity.modifiers,
                    actor,
                    &mut state.effect_queue,
                )
            }
        }
        EffectKind::MoveUpdate => {
            let monster = target.unwrap();
            let entity = &mut state.entities[monster];
            process_effect_move_update::process_effect_move_update(entity, &mut state.rng)
        }
        EffectKind::RoomEnter => process_effect_room_enter::process_effect_room_enter(
            &state.map,
            state.ascension,
            &mut state.entities,
            &mut state.monsters,
            &mut state.monster_count,
            &mut state.rng,
            &mut state.effect_queue,
        ),
        EffectKind::RestSiteExit => process_effect_rest_site_exit::process_effect_rest_site_exit(
            &mut state.map,
            &mut state.effect_queue,
        ),
        // Halt-kind variants: represent pending player decisions.
        // RoomSelect and CardRewardSelect in their `Direct` form (after
        // the resolver picked a target) complete the transition. Before
        // resolution they're handled by the `Resolve` branch in `process_effect`.
        EffectKind::RoomSelect => {
            let node_id = target.expect("RoomSelect Direct form must have target");
            let node = &state.entities[node_id];
            state.map.position = Position::Overworld {
                y: node.node_y,
                x: node.node_x,
            };
            state
                .effect_queue
                .push_front(Effect::direct(EffectKind::RoomEnter, None, None));
            DispatchResult::Continue
        }
        EffectKind::CardRewardSelect => {
            let card_id = target.expect("CardRewardSelect Direct form must have target");
            state.deck.push(card_id);
            state
                .effect_queue
                .push_front(Effect::direct(EffectKind::CardRewardClear, None, None));
            DispatchResult::Continue
        }
        EffectKind::GameOver => DispatchResult::Halt {
            phase_new: Phase::GameOver,
        },
        EffectKind::AwaitCombatAction => DispatchResult::Halt {
            phase_new: Phase::CombatDefault,
        },
        EffectKind::AwaitRestSiteAction => DispatchResult::Halt {
            phase_new: Phase::RestSite,
        },
        EffectKind::AwaitCardRewardRoll => DispatchResult::Halt {
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
            DispatchResult::Continue => {}
            DispatchResult::Halt { phase_new } => {
                state.phase = phase_new;
                return;
            }
        }
    }
}
