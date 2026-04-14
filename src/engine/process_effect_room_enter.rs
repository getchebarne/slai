use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::monsters::spawn_monster;
use crate::entities::Monster;
use crate::state::Map;
use crate::entities::{Entity, EntityKind};
use crate::types::{MonsterName, RoomType};
use crate::map::{active_room_type};

fn push_monster(
    monster: Monster,
    entities: &mut Vec<Entity>,
    monsters: &mut [usize],
    monster_count: &mut u8,
) {
    let id = entities.len();
    entities.push(Entity {
        kind: EntityKind::Monster(monster),
    });
    monsters[*monster_count as usize] = id;
    *monster_count += 1;
}

pub fn process_effect_room_enter(
    map: &Map,
    ascension: u8,
    entities: &mut Vec<Entity>,
    monsters: &mut [usize],
    monster_count: &mut u8,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    *monster_count = 0;

    // Spawn monsters based on room type
    let room = active_room_type(&map, entities).unwrap();
    match room {
        RoomType::CombatBoss => {
            // TODO: other bosses
            let m = spawn_monster(MonsterName::TheGuardian, ascension, rng);
            push_monster(m, entities, monsters, monster_count);
            ProcessEffectResult::AddAndContinue {
                top: vec![Effect {
                    kind: EffectKind::CombatStart,
                    source: None,
                    target: Target::Direct(None),
                }],
                bot: Vec::new(),
            }
        }
        RoomType::CombatMonster => {
            let encounter: u8 = rng.random_range(0..3);
            match encounter {
                0 => {
                    let m = spawn_monster(MonsterName::JawWorm, ascension, rng);
                    push_monster(m, entities, monsters, monster_count);
                }
                1 => {
                    let m = spawn_monster(MonsterName::Cultist, ascension, rng);
                    push_monster(m, entities, monsters, monster_count);
                }
                2 => {
                    let m1 = spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    let m2 = spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    push_monster(m1, entities, monsters, monster_count);
                    push_monster(m2, entities, monsters, monster_count);
                }
                _ => unreachable!(),
            };
            ProcessEffectResult::AddAndContinue {
                top: vec![Effect {
                    kind: EffectKind::CombatStart,
                    source: None,
                    target: Target::Direct(None),
                }],
                bot: Vec::new(),
            }
        }
        RoomType::RestSite => ProcessEffectResult::AddAndContinue {
            top: vec![Effect::direct(EffectKind::AwaitRestSiteAction, None, None)],
            bot: Vec::new(),
        },
    }
}
