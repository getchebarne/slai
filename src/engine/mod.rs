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
use crate::state::{GameState, Location};
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
    kind: EffectKind::Noop,
    id_source: None,
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
    id_source: usize,
    id_character: usize,
    id_hand: &[usize],
    id_card_target: Option<usize>,
    id_alive_monsters: &[usize],
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    id_card_rewards: &[usize],
    buf_cands: &mut CandidateBuf,
) {
    match candidates {
        CandidatePool::Hand => buf_cands.extend_from_slice(id_hand),
        CandidatePool::CardTarget => buf_cands.push(id_card_target.unwrap()),
        CandidatePool::Character => buf_cands.push(id_character),
        CandidatePool::Monsters => buf_cands.extend_from_slice(id_alive_monsters),
        CandidatePool::Source => buf_cands.push(id_source),
        CandidatePool::CardRewardPool => buf_cands.extend_from_slice(id_card_rewards),
        CandidatePool::NextRowRooms => match location {
            Location::Start => {
                for col in 0..MAP_WIDTH {
                    if let Some(id_room) = id_rooms[0][col] {
                        buf_cands.push(id_room);
                    }
                }
            }
            Location::Overworld { y, x } => {
                let y_next = y + 1;
                if y_next >= MAP_HEIGHT {
                    return;
                }
                if let Some(id_current) = id_rooms[y][x] {
                    let current_room = &entities[id_current];
                    for col in 0..MAP_WIDTH {
                        if has_edge(current_room.edges, col) {
                            if let Some(id_room) = id_rooms[y_next][col] {
                                buf_cands.push(id_room);
                            }
                        }
                    }
                }
            }
            Location::BossRoom => {}
        },
    }
}

fn resolve_targets(
    candidates: CandidatePool,
    selection: SelectionKind,
    id_source: usize,
    id_character: usize,
    id_hand: &[usize],
    id_card_target: Option<usize>,
    id_alive_monsters: &[usize],
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    id_card_rewards: &[usize],
    rng: &mut impl Rng,
    buf_cands: &mut CandidateBuf,
) -> TargetResolution {
    resolve_candidates(
        candidates,
        id_source,
        id_character,
        id_hand,
        id_card_target,
        id_alive_monsters,
        id_rooms,
        location,
        entities,
        id_card_rewards,
        buf_cands,
    );
    match selection {
        SelectionKind::All => TargetResolution::Resolved,
        SelectionKind::Random { count } => {
            shuffle(buf_cands.as_mut_slice(), rng);
            buf_cands.truncate(count as usize);
            TargetResolution::Resolved
        }
        SelectionKind::Input { count } => {
            if count as usize >= buf_cands.len {
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
    let id_target = match effect.target {
        Target::Direct(t) => t,
        Target::Resolve {
            candidates,
            selection,
        } => {
            return resolve_or_halt(state, effect.kind, effect.id_source, candidates, selection);
        }
    };

    dispatch_by_kind(state, effect.kind, effect.id_source, id_target)
}

fn resolve_or_halt(
    state: &mut GameState,
    kind: EffectKind,
    id_source: Option<usize>,
    candidates: CandidatePool,
    selection: SelectionKind,
) -> DispatchResult {
    // Stack locals
    let mut buf_alive = [0usize; MAX_MONSTERS];
    let mut buf_cands = CandidateBuf::new();

    let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
    let id_source_resolved = id_source.unwrap_or(state.id_character);
    let resolution = resolve_targets(
        candidates,
        selection,
        id_source_resolved,
        state.id_character,
        &state.id_hand,
        state.id_card_target,
        &buf_alive[..alive_n],
        &state.id_rooms,
        state.location,
        &state.entities,
        &state.id_card_rewards,
        &mut state.rng,
        &mut buf_cands,
    );
    match resolution {
        TargetResolution::Resolved => {
            // Push one Direct-target effect per resolved id. push_front reverses
            // order, so iterate in reverse to preserve id order in the queue.
            for &id_target in buf_cands.as_slice().iter().rev() {
                state.effect_queue.push_front(Effect {
                    kind,
                    id_source,
                    target: Target::Direct(Some(id_target)),
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
    id_source: Option<usize>,
    id_target: Option<usize>,
) -> DispatchResult {
    match kind {
        EffectKind::CardDraw { count } => process_effect_card_draw::process_effect_card_draw(
            count,
            &mut state.id_draw_pile,
            &mut state.id_hand,
            &mut state.id_discard_pile,
            &mut state.rng,
        ),
        EffectKind::CardPlay => {
            let id_card = id_target.unwrap();
            // Stack locals
            let mut buf_alive = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
            process_effect_card_play::process_effect_card_play(
                id_card,
                state.id_card_target,
                state.id_character,
                &state.entities,
                &state.id_hand,
                &buf_alive[..alive_n],
                &mut state.rng,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardDiscard => {
            let id_card = id_target.unwrap();
            process_effect_card_discard::process_effect_card_discard(
                id_card,
                &mut state.id_hand,
                &mut state.id_discard_pile,
            )
        }
        EffectKind::CardExhaust => {
            let id_card = id_target.unwrap();
            process_effect_card_exhaust::process_effect_card_exhaust(
                id_card,
                &mut state.id_hand,
                &mut state.id_exhaust_pile,
            )
        }
        EffectKind::CardRemove => {
            let id_card = id_target.unwrap();
            process_effect_card_remove::process_effect_card_remove(id_card, &mut state.id_hand)
        }
        EffectKind::AddShivs { count } => process_effect_add_shivs::process_effect_add_shivs(
            count,
            &mut state.entities,
            &mut state.id_hand,
            &mut state.id_discard_pile,
        ),
        EffectKind::CalculatedGamble => {
            process_effect_calculated_gamble::process_effect_calculated_gamble(
                &state.id_hand,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardUpgrade => {
            let id_card = id_target.unwrap();
            process_effect_card_upgrade::process_effect_card_upgrade(id_card, &mut state.entities)
        }
        EffectKind::CardRewardRoll => {
            process_effect_card_reward_roll::process_effect_card_reward_roll(
                state.id_character,
                &mut state.id_card_rewards,
                &mut state.entities,
                &mut state.rng,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardRewardClear => {
            process_effect_card_reward_clear::process_effect_card_reward_clear(
                &mut state.id_card_rewards,
                &mut state.effect_queue,
            )
        }
        EffectKind::TargetSet => {
            let id_target = id_target.unwrap();
            process_effect_target_set::process_effect_target_set(
                &mut state.id_card_target,
                id_target,
            )
        }
        EffectKind::TargetClear => {
            process_effect_target_clear::process_effect_target_clear(&mut state.id_card_target)
        }
        EffectKind::DamagePhysical { base } => {
            let id_source = id_source.unwrap();
            let id_target = id_target.unwrap();
            let source_mods = &state.entities[id_source].modifiers;
            let target_mods = &state.entities[id_target].modifiers;
            process_effect_damage_physical::process_effect_damage_physical(
                source_mods,
                target_mods,
                id_target,
                base,
                &mut state.effect_queue,
            )
        }
        EffectKind::DamageDeal { amount } => {
            let id_target = id_target.unwrap();
            let vitals = &mut state.entities[id_target].vitals;
            process_effect_damage_deal::process_effect_damage_deal(
                vitals,
                id_target,
                amount,
                &mut state.effect_queue,
            )
        }
        EffectKind::HealthGain { amount } => {
            let id_target = id_target.unwrap();
            let vitals = &mut state.entities[id_target].vitals;
            process_effect_health_gain::process_effect_health_gain(vitals, amount)
        }
        EffectKind::HealthLoss { amount } => {
            let id_target = id_target.unwrap();
            let entity = &mut state.entities[id_target];
            process_effect_health_loss::process_effect_health_loss(
                &mut entity.vitals,
                &mut entity.modifiers,
                id_target,
                state.id_character,
                amount,
                &mut state.effect_queue,
            )
        }
        EffectKind::BlockGain { amount } => {
            let id_target = id_target.unwrap();
            let from_card = match id_source {
                Some(id) => state.entities[id].kind == EntityType::Card,
                None => false,
            };
            let entity = &mut state.entities[id_target];
            process_effect_block_gain::process_effect_block_gain(
                &mut entity.vitals,
                &mut entity.modifiers,
                amount,
                from_card,
            )
        }
        EffectKind::BlockSet { amount } => {
            let id_target = id_target.unwrap();
            let vitals = &mut state.entities[id_target].vitals;
            process_effect_block_set::process_effect_block_set(vitals, amount)
        }
        EffectKind::EnergyGain { amount } => {
            process_effect_energy_gain::process_effect_energy_gain(&mut state.energy, amount)
        }
        EffectKind::EnergyLoss { amount } => {
            process_effect_energy_loss::process_effect_energy_loss(&mut state.energy, amount)
        }
        EffectKind::ModifierGain { kind, stacks } => {
            let id_target = id_target.unwrap();
            let entity = &mut state.entities[id_target];
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
            let id_target = id_target.unwrap();
            let modifiers = &mut state.entities[id_target].modifiers;
            process_effect_modifier_remove::process_effect_modifier_remove(modifiers, kind)
        }
        EffectKind::ModifierTick => {
            let id_target = id_target.unwrap();
            let modifiers = &mut state.entities[id_target].modifiers;
            process_effect_modifier_tick::process_effect_modifier_tick(modifiers)
        }
        EffectKind::ModifierSetNotNew => {
            // Stack locals
            let mut buf_alive = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
            process_effect_modifier_set_not_new::process_effect_modifier_set_not_new(
                state.id_character,
                &mut state.entities,
                &buf_alive[..alive_n],
            )
        }
        EffectKind::Death => {
            let id_actor = id_target.unwrap();
            process_effect_death::process_effect_death(
                id_actor,
                state.id_character,
                &state.id_monsters,
                state.monster_count,
                &mut state.entities,
                &mut state.effect_queue,
            )
        }
        EffectKind::CombatStart => process_effect_combat_start::process_effect_combat_start(
            state.id_character,
            &state.id_deck,
            &mut state.entities,
            &mut state.id_draw_pile,
            &mut state.id_hand,
            &mut state.id_discard_pile,
            &mut state.id_exhaust_pile,
            &mut state.id_card_target,
            &state.id_monsters,
            state.monster_count,
            &mut state.rng,
            &mut state.effect_queue,
        ),
        EffectKind::CombatEnd => process_effect_combat_end::process_effect_combat_end(
            state.id_character,
            &mut state.id_hand,
            &mut state.id_draw_pile,
            &mut state.id_discard_pile,
            &mut state.id_exhaust_pile,
            &mut state.id_card_target,
            &mut state.entities,
            &mut state.monster_count,
            &state.id_rooms,
            state.location,
            &mut state.effect_queue,
        ),
        EffectKind::TurnStart => {
            let id_actor = id_target.unwrap();

            // Stack locals
            let mut buf_alive = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
            let entity = &mut state.entities[id_actor];
            process_effect_turn_start::process_effect_turn_start(
                &mut entity.vitals,
                &mut entity.modifiers,
                id_actor,
                state.id_character,
                &state.energy,
                &buf_alive[..alive_n],
                &mut state.effect_queue,
            )
        }
        EffectKind::TurnEnd => {
            let id_actor = id_target.unwrap();
            if id_actor == state.id_character {
                // Stack locals
                let mut buf_alive = [0usize; MAX_MONSTERS];
                let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
                process_effect_turn_end::process_effect_turn_end_character(
                    state.id_character,
                    &state.entities,
                    &state.id_hand,
                    state.id_card_target,
                    &buf_alive[..alive_n],
                    &mut state.rng,
                    &mut state.effect_queue,
                )
            } else {
                let entity = &mut state.entities[id_actor];
                process_effect_turn_end::process_effect_turn_end_monster(
                    &mut entity.vitals,
                    &mut entity.modifiers,
                    id_actor,
                    &mut state.effect_queue,
                )
            }
        }
        EffectKind::MoveUpdate => {
            let id_monster = id_target.unwrap();
            let entity = &mut state.entities[id_monster];
            process_effect_move_update::process_effect_move_update(entity, &mut state.rng)
        }
        EffectKind::RoomEnter => process_effect_room_enter::process_effect_room_enter(
            &state.id_rooms,
            state.location,
            state.ascension,
            &mut state.entities,
            &mut state.id_monsters,
            &mut state.monster_count,
            &mut state.rng,
            &mut state.effect_queue,
        ),
        EffectKind::RestSiteExit => process_effect_rest_site_exit::process_effect_rest_site_exit(
            &mut state.location,
            &mut state.effect_queue,
        ),
        // Halt-kind variants: represent pending player decisions.
        // RoomSelect and CardRewardSelect in their `Direct` form (after
        // the resolver picked a target) complete the transition. Before
        // resolution they're handled by the `Resolve` branch in `process_effect`.
        EffectKind::RoomSelect => {
            let id_room = id_target.expect("RoomSelect Direct form must have target");
            let room = &state.entities[id_room];
            state.location = Location::Overworld {
                y: room.room_y,
                x: room.room_x,
            };
            state
                .effect_queue
                .push_front(Effect::direct(EffectKind::RoomEnter, None, None));
            DispatchResult::Continue
        }
        EffectKind::CardRewardSelect => {
            let id_card = id_target.expect("CardRewardSelect Direct form must have target");
            state.id_deck.push(id_card);
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
        EffectKind::Noop => panic!("Noop effect should never be dispatched"),
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
