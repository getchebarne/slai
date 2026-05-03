use std::collections::VecDeque;

use rand::Rng;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::map::active_room_kind;
use crate::monsters::spawn_monster;
use crate::state::Location;
use crate::types::{MonsterName, RoomKind};
use crate::utils::shuffle;

fn push_monster(
    monster: Entity,
    entities: &mut Vec<Entity>,
    id_monsters: &mut [usize],
    monster_count: &mut u8,
) {
    let id_monster = entities.len();
    entities.push(monster);
    id_monsters[*monster_count as usize] = id_monster;
    *monster_count += 1;
}

pub fn process_effect_room_enter(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    ascension: u8,
    entities: &mut Vec<Entity>,
    id_monsters: &mut [usize],
    monster_count: &mut u8,
    rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    *monster_count = 0;

    let room = active_room_kind(id_rooms, location, entities).unwrap();
    match room {
        RoomKind::CombatBoss => {
            // Act 1 boss: 50/50 between TheGuardian and SlimeBoss
            // TODO: Hexaghost
            let name = if rng.random_bool(0.5) {
                MonsterName::TheGuardian
            } else {
                MonsterName::SlimeBoss
            };
            let m = spawn_monster(name, ascension, rng);
            push_monster(m, entities, id_monsters, monster_count);
            queue.push_front(Effect {
                kind: EffectKind::CombatStart,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::CombatMonster => {
            let encounter: u8 = rng.random_range(0..14);
            match encounter {
                0 => {
                    let monster = spawn_monster(MonsterName::JawWorm, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                1 => {
                    let monster = spawn_monster(MonsterName::Cultist, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                2 => {
                    let monster_1 = spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    let monster_2 = spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    push_monster(monster_1, entities, id_monsters, monster_count);
                    push_monster(monster_2, entities, id_monsters, monster_count);
                }
                3 => {
                    // Small Slimes: 50/50 between [Spike_S + Acid_M] and [Acid_S + Spike_M].
                    // Java MonsterHelper.spawnSmallSlimes (one small + one medium of opposite color).
                    let (small, medium) = if rng.random_bool(0.5) {
                        (MonsterName::SlimeSpikeSmall, MonsterName::SlimeAcidMedium)
                    } else {
                        (MonsterName::SlimeAcidSmall, MonsterName::SlimeSpikeMedium)
                    };
                    let monster_1 = spawn_monster(small, ascension, rng);
                    let monster_2 = spawn_monster(medium, ascension, rng);
                    push_monster(monster_1, entities, id_monsters, monster_count);
                    push_monster(monster_2, entities, id_monsters, monster_count);
                }
                4 => {
                    let monster = spawn_monster(MonsterName::SlaverBlue, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                5 => {
                    // Gremlin Gang: 4 gremlins drawn without replacement
                    let mut pool: [MonsterName; 8] = [
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
                        let monster = spawn_monster(name, ascension, rng);
                        push_monster(monster, entities, id_monsters, monster_count);
                    }
                }
                6 => {
                    // 2 Louse: independent draws from {Red, Green}
                    static POOL: &[MonsterName] =
                        &[MonsterName::LouseNormal, MonsterName::LouseDefensive];
                    for _ in 0..2 {
                        let monster_name = POOL[rng.random_range(0..POOL.len())];
                        let monster = spawn_monster(monster_name, ascension, rng);
                        push_monster(monster, entities, id_monsters, monster_count);
                    }
                }
                7 => {
                    // 3 Louse: independent draws from {Red, Green}
                    static POOL: &[MonsterName] =
                        &[MonsterName::LouseNormal, MonsterName::LouseDefensive];
                    for _ in 0..3 {
                        let monster_name = POOL[rng.random_range(0..POOL.len())];
                        let monster = spawn_monster(monster_name, ascension, rng);
                        push_monster(monster, entities, id_monsters, monster_count);
                    }
                }
                8 => {
                    let monster = spawn_monster(MonsterName::SlimeAcidMedium, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                9 => {
                    let monster = spawn_monster(MonsterName::SlimeSpikeMedium, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                10 => {
                    // Large Slime: 50/50 between Acid_L and Spike_L.
                    let name = if rng.random_bool(0.5) {
                        MonsterName::SlimeAcidLarge
                    } else {
                        MonsterName::SlimeSpikeLarge
                    };
                    let monster = spawn_monster(name, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                11 => {
                    // Lots of Slimes: 5 small slimes drawn without replacement
                    // from [Spike_S×3, Acid_S×2]. Composition is fixed
                    // (3 Spike_S + 2 Acid_S); only spawn order varies.
                    let mut pool: [MonsterName; 5] = [
                        MonsterName::SlimeSpikeSmall,
                        MonsterName::SlimeSpikeSmall,
                        MonsterName::SlimeSpikeSmall,
                        MonsterName::SlimeAcidSmall,
                        MonsterName::SlimeAcidSmall,
                    ];
                    shuffle(&mut pool, rng);
                    for &name in &pool {
                        let monster = spawn_monster(name, ascension, rng);
                        push_monster(monster, entities, id_monsters, monster_count);
                    }
                }
                12 => {
                    let monster = spawn_monster(MonsterName::Looter, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                13 => {
                    let monster = spawn_monster(MonsterName::SlaverRed, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                _ => unreachable!(),
            };
            queue.push_front(Effect {
                kind: EffectKind::CombatStart,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::CombatElite => {
            // Sentries, GremlinNob, or Lagavulin
            let pick: u8 = rng.random_range(0..3);
            match pick {
                0 => {
                    let monster_1 = spawn_monster(MonsterName::Sentry, ascension, rng);
                    let monster_2 = spawn_monster(MonsterName::Sentry, ascension, rng);
                    let monster_3 = spawn_monster(MonsterName::Sentry, ascension, rng);
                    push_monster(monster_1, entities, id_monsters, monster_count);
                    push_monster(monster_2, entities, id_monsters, monster_count);
                    push_monster(monster_3, entities, id_monsters, monster_count);
                }
                1 => {
                    let monster = spawn_monster(MonsterName::GremlinNob, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                2 => {
                    let monster = spawn_monster(MonsterName::Lagavulin, ascension, rng);
                    push_monster(monster, entities, id_monsters, monster_count);
                }
                _ => unreachable!(),
            }
            queue.push_front(Effect {
                kind: EffectKind::CombatStart,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite => {
            // Nothing to enqueue; the queue drains and the engine derives
            // Phase::RestSite from `location` + room kind
        }
    }
    DispatchResult::Continue
}
