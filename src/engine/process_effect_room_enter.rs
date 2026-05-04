use std::collections::VecDeque;

use rand::Rng;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH};
use crate::effect::{Effect, EffectKind};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::game::Location;
use crate::map::active_room_kind;
use crate::types::{MonsterName, RoomKind};
use crate::utils::shuffle;

pub fn process_effect_room_enter(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    monster_count: &mut u8,
    rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    *monster_count = 0;

    let room = active_room_kind(id_rooms, location, entities).unwrap();
    let mut effects: Vec<Effect> = Vec::new();
    match room {
        RoomKind::CombatBoss => {
            // Act 1 boss: uniform 1/3 between The Guardian, Slime Boss, Hexaghost
            let pick: u8 = rng.random_range(0..3);
            let name = match pick {
                0 => MonsterName::TheGuardian,
                1 => MonsterName::SlimeBoss,
                2 => MonsterName::Hexaghost,
                _ => unreachable!(),
            };
            effects.push(Effect::direct(
                EffectKind::MonsterSpawn { name },
                None,
                None,
            ));
        }
        RoomKind::CombatMonster => {
            let encounter: u8 = rng.random_range(0..14);
            match encounter {
                0 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::JawWorm,
                        },
                        None,
                        None,
                    ));
                }
                1 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::Cultist,
                        },
                        None,
                        None,
                    ));
                }
                2 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::FungiBeast,
                        },
                        None,
                        None,
                    ));
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::FungiBeast,
                        },
                        None,
                        None,
                    ));
                }
                3 => {
                    // Small Slimes: 50/50 between [Spike_S + Acid_M] and [Acid_S + Spike_M]
                    let (small, medium) = if rng.random_bool(0.5) {
                        (MonsterName::SlimeSpikeSmall, MonsterName::SlimeAcidMedium)
                    } else {
                        (MonsterName::SlimeAcidSmall, MonsterName::SlimeSpikeMedium)
                    };
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn { name: small },
                        None,
                        None,
                    ));
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn { name: medium },
                        None,
                        None,
                    ));
                }
                4 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::SlaverBlue,
                        },
                        None,
                        None,
                    ));
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
                        effects.push(Effect::direct(
                            EffectKind::MonsterSpawn { name },
                            None,
                            None,
                        ));
                    }
                }
                6 => {
                    // 2 Louse: independent draws from {Red, Green}
                    static POOL: &[MonsterName] =
                        &[MonsterName::LouseNormal, MonsterName::LouseDefensive];
                    for _ in 0..2 {
                        let name = POOL[rng.random_range(0..POOL.len())];
                        effects.push(Effect::direct(
                            EffectKind::MonsterSpawn { name },
                            None,
                            None,
                        ));
                    }
                }
                7 => {
                    // 3 Louse: independent draws from {Red, Green}
                    static POOL: &[MonsterName] =
                        &[MonsterName::LouseNormal, MonsterName::LouseDefensive];
                    for _ in 0..3 {
                        let name = POOL[rng.random_range(0..POOL.len())];
                        effects.push(Effect::direct(
                            EffectKind::MonsterSpawn { name },
                            None,
                            None,
                        ));
                    }
                }
                8 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::SlimeAcidMedium,
                        },
                        None,
                        None,
                    ));
                }
                9 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::SlimeSpikeMedium,
                        },
                        None,
                        None,
                    ));
                }
                10 => {
                    // Large Slime: 50/50 between Acid_L and Spike_L.
                    let name = if rng.random_bool(0.5) {
                        MonsterName::SlimeAcidLarge
                    } else {
                        MonsterName::SlimeSpikeLarge
                    };
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn { name },
                        None,
                        None,
                    ));
                }
                11 => {
                    // Lots of Slimes: 5 small slimes drawn without replacement
                    let mut pool: [MonsterName; 5] = [
                        MonsterName::SlimeSpikeSmall,
                        MonsterName::SlimeSpikeSmall,
                        MonsterName::SlimeSpikeSmall,
                        MonsterName::SlimeAcidSmall,
                        MonsterName::SlimeAcidSmall,
                    ];
                    shuffle(&mut pool, rng);
                    for &name in &pool {
                        effects.push(Effect::direct(
                            EffectKind::MonsterSpawn { name },
                            None,
                            None,
                        ));
                    }
                }
                12 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::Looter,
                        },
                        None,
                        None,
                    ));
                }
                13 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::SlaverRed,
                        },
                        None,
                        None,
                    ));
                }
                _ => unreachable!(),
            };
        }
        RoomKind::CombatElite => {
            // Sentries, GremlinNob, or Lagavulin
            let pick: u8 = rng.random_range(0..3);
            match pick {
                0 => {
                    for _ in 0..3 {
                        effects.push(Effect::direct(
                            EffectKind::MonsterSpawn {
                                name: MonsterName::Sentry,
                            },
                            None,
                            None,
                        ));
                    }
                }
                1 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::GremlinNob,
                        },
                        None,
                        None,
                    ));
                }
                2 => {
                    effects.push(Effect::direct(
                        EffectKind::MonsterSpawn {
                            name: MonsterName::Lagavulin,
                        },
                        None,
                        None,
                    ));
                }
                _ => unreachable!(),
            }
        }
        RoomKind::RestSite => {
            // Nothing to enqueue; the queue drains and the engine derives
            // Phase::RestSite from `location` + room kind
        }
    }

    if !effects.is_empty() {
        effects.push(Effect::direct(EffectKind::CombatStart, None, None));
        for effect in effects.into_iter().rev() {
            queue.push_front(effect);
        }
    }

    DispatchResult::Continue
}
