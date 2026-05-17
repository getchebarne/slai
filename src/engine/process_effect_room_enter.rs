use std::collections::VecDeque;

use rand::Rng;

use crate::consts::CHEST_SMALL_PCT;
use crate::consts::CHEST_SMALL_PLUS_MEDIUM_PCT;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::engine::EffectBuf;
use crate::entity::Entity;
use crate::events::POOL_ACT1_EVENT;
use crate::events::spawn_event;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::map::room_at_mut;
use crate::types::ChestKind;
use crate::types::EventName;
use crate::types::MonsterEncounter;
use crate::types::MonsterName;
use crate::types::Phase;
use crate::types::RoomKind;
use crate::utils::shuffle;

pub fn process_effect_room_enter(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &mut Vec<Entity>,
    encounter_list_normal: &mut Vec<MonsterEncounter>,
    encounter_list_elite: &mut Vec<MonsterEncounter>,
    encounter_boss: MonsterEncounter,
    events_seen_this_run: &mut Vec<EventName>,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let room_kind = get_active_room_kind(id_rooms, location, entities).unwrap();
    let mut buf_effects = EffectBuf::new();

    match room_kind {
        RoomKind::CombatBoss => {
            spawn_encounter_monsters(encounter_boss, &mut buf_effects, rng);
        }
        RoomKind::CombatMonster => {
            let encounter = encounter_list_normal.remove(0);
            spawn_encounter_monsters(encounter, &mut buf_effects, rng);
        }
        RoomKind::CombatElite => {
            let encounter = encounter_list_elite.remove(0);
            spawn_encounter_monsters(encounter, &mut buf_effects, rng);
        }
        RoomKind::RestSite => {
            // Nothing to enqueue; derive_phase reads Location & RoomKind for RestSite
        }
        RoomKind::Treasure => {
            let Location::Overworld { y, x } = location else {
                unreachable!("RoomEnter on Treasure outside Overworld");
            };
            let room = room_at_mut(id_rooms, entities, y, x).expect("Treasure room missing");
            let roll = rng.random_range(0..100) as u8;
            room.room_chest_kind = Some(if roll < CHEST_SMALL_PCT {
                ChestKind::Small
            } else if roll < CHEST_SMALL_PLUS_MEDIUM_PCT {
                ChestKind::Medium
            } else {
                ChestKind::Large
            });
        }
        RoomKind::EventRoom => {
            if let Some(phase) = spawn_random_event(entities, events_seen_this_run, rng) {
                return Some(phase);
            }
        }
        RoomKind::Shop => {}
    }

    if buf_effects.len > 0 {
        buf_effects.push(Effect::direct(EffectKind::CombatStart, None, None));
        buf_effects.push_all_front(effect_queue);
    }

    None
}

// Drift-state ? resolution is deferred until shop/surprise-treasure paths exist
fn spawn_random_event(
    entities: &mut Vec<Entity>,
    events_seen_this_run: &mut Vec<EventName>,
    rng: &mut impl Rng,
) -> Option<Phase> {
    if POOL_ACT1_EVENT.is_empty() {
        return None;
    }
    if events_seen_this_run.len() >= POOL_ACT1_EVENT.len() {
        events_seen_this_run.clear();
    }
    let name = loop {
        let cand = POOL_ACT1_EVENT[rng.random_range(0..POOL_ACT1_EVENT.len())];
        if !events_seen_this_run.contains(&cand) {
            break cand;
        }
    };
    events_seen_this_run.push(name);
    let id_event = entities.len();
    entities.push(spawn_event(name, rng));
    Some(Phase::EventChoice { id_event })
}

fn pick_louse(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::LouseNormal
    } else {
        MonsterName::LouseDefensive
    }
}

fn pick_slaver(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::SlaverRed
    } else {
        MonsterName::SlaverBlue
    }
}

fn pick_wildlife_weak(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => pick_louse(rng),
        1 => MonsterName::SlimeSpikeMedium,
        2 => MonsterName::SlimeAcidMedium,
        _ => unreachable!(),
    }
}

fn pick_wildlife_strong(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::FungiBeast
    } else {
        MonsterName::JawWorm
    }
}

fn pick_humanoid_strong(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => MonsterName::Cultist,
        1 => pick_slaver(rng),
        2 => MonsterName::Looter,
        _ => unreachable!(),
    }
}

#[inline]
fn push_monster_spawn(effects: &mut EffectBuf, name: MonsterName) {
    effects.push(Effect::direct(
        EffectKind::MonsterSpawn { name },
        None,
        None,
    ));
}

fn spawn_encounter_monsters(
    encounter: MonsterEncounter,
    effects: &mut EffectBuf,
    rng: &mut impl Rng,
) {
    match encounter {
        MonsterEncounter::Cultist => push_monster_spawn(effects, MonsterName::Cultist),
        MonsterEncounter::JawWorm => push_monster_spawn(effects, MonsterName::JawWorm),
        MonsterEncounter::TwoLouse => {
            for _ in 0..2 {
                push_monster_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::SmallSlimes => {
            let (small, medium) = if rng.random_bool(0.5) {
                (MonsterName::SlimeSpikeSmall, MonsterName::SlimeAcidMedium)
            } else {
                (MonsterName::SlimeAcidSmall, MonsterName::SlimeSpikeMedium)
            };
            push_monster_spawn(effects, small);
            push_monster_spawn(effects, medium);
        }
        MonsterEncounter::BlueSlaver => push_monster_spawn(effects, MonsterName::SlaverBlue),
        MonsterEncounter::RedSlaver => push_monster_spawn(effects, MonsterName::SlaverRed),
        MonsterEncounter::Looter => push_monster_spawn(effects, MonsterName::Looter),
        MonsterEncounter::TwoFungiBeasts => {
            push_monster_spawn(effects, MonsterName::FungiBeast);
            push_monster_spawn(effects, MonsterName::FungiBeast);
        }
        MonsterEncounter::ThreeLouse => {
            for _ in 0..3 {
                push_monster_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::LargeSlime => {
            let name = if rng.random_bool(0.5) {
                MonsterName::SlimeAcidLarge
            } else {
                MonsterName::SlimeSpikeLarge
            };
            push_monster_spawn(effects, name);
        }
        MonsterEncounter::LotsOfSlimes => {
            let mut pool = [
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeAcidSmall,
                MonsterName::SlimeAcidSmall,
            ];
            shuffle(&mut pool, rng);
            for &name in &pool {
                push_monster_spawn(effects, name);
            }
        }
        MonsterEncounter::GremlinGang => {
            let mut pool = [
                MonsterName::GremlinWarrior,
                MonsterName::GremlinWarrior,
                MonsterName::GremlinThief,
                MonsterName::GremlinThief,
                MonsterName::GremlinFat,
                MonsterName::GremlinFat,
                MonsterName::GremlinTsundere,
                MonsterName::GremlinWizard,
            ];
            shuffle(&mut pool, rng);
            for &name in &pool[..4] {
                push_monster_spawn(effects, name);
            }
        }
        MonsterEncounter::ExordiumThugs => {
            push_monster_spawn(effects, pick_wildlife_weak(rng));
            push_monster_spawn(effects, pick_humanoid_strong(rng));
        }
        MonsterEncounter::ExordiumWildlife => {
            push_monster_spawn(effects, pick_wildlife_strong(rng));
            push_monster_spawn(effects, pick_wildlife_weak(rng));
        }
        MonsterEncounter::GremlinNob => push_monster_spawn(effects, MonsterName::GremlinNob),
        MonsterEncounter::Lagavulin => push_monster_spawn(effects, MonsterName::Lagavulin),
        MonsterEncounter::ThreeSentries => {
            for _ in 0..3 {
                push_monster_spawn(effects, MonsterName::Sentry);
            }
        }
        MonsterEncounter::TheGuardian => push_monster_spawn(effects, MonsterName::TheGuardian),
        MonsterEncounter::Hexaghost => push_monster_spawn(effects, MonsterName::Hexaghost),
        MonsterEncounter::SlimeBoss => push_monster_spawn(effects, MonsterName::SlimeBoss),
    }
}
