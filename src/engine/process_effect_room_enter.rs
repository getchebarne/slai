use rand::Rng;

use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::monsters;
use crate::state::{Entity, EntityKind, Map};
use crate::types::{EntityId, MonsterName, RoomType};

fn push_monster(
    monster: crate::monsters::Monster,
    entities: &mut Vec<Entity>,
    monsters: &mut [EntityId],
    monster_count: &mut u8,
) {
    let id = EntityId(entities.len() as u32);
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
    monsters: &mut [EntityId],
    monster_count: &mut u8,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    *monster_count = 0;

    let room = map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => {
            let m = monsters::spawn_monster(MonsterName::TheGuardian, ascension, rng);
            push_monster(m, entities, monsters, monster_count);
            ProcessEffectResult::Continue {
                top: vec![Effect {
                    kind: EffectKind::CombatStart,
                    source: None,
                    target: None,
                }],
                bot: Vec::new(),
            }
        }
        RoomType::CombatMonster => {
            let encounter: u8 = rng.random_range(0..3);
            match encounter {
                0 => {
                    let m = monsters::spawn_monster(MonsterName::JawWorm, ascension, rng);
                    push_monster(m, entities, monsters, monster_count);
                }
                1 => {
                    let m = monsters::spawn_monster(MonsterName::Cultist, ascension, rng);
                    push_monster(m, entities, monsters, monster_count);
                }
                2 => {
                    let m1 = monsters::spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    let m2 = monsters::spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    push_monster(m1, entities, monsters, monster_count);
                    push_monster(m2, entities, monsters, monster_count);
                }
                _ => unreachable!(),
            };
            ProcessEffectResult::Continue {
                top: vec![Effect {
                    kind: EffectKind::CombatStart,
                    source: None,
                    target: None,
                }],
                bot: Vec::new(),
            }
        }
        RoomType::RestSite => ProcessEffectResult::Pass,
    }
}
