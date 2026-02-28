use rand::Rng;

use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::monsters;
use crate::state::{Entity, EntityKind, Map};
use crate::types::{MonsterName, RoomType};

pub fn process_effect_room_enter(
    map: &Map,
    ascension: u8,
    entities: &mut Vec<Option<Entity>>,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    let room = map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => {
            let m = monsters::spawn_monster(MonsterName::TheGuardian, ascension, rng);
            entities.push(Some(Entity { kind: EntityKind::Monster(m) }));
            ProcessEffectResult::Continue {
                top: vec![Effect::CombatStart],
                bot: Vec::new(),
            }
        }
        RoomType::CombatMonster => {
            let encounter: u8 = rng.random_range(0..3);
            match encounter {
                0 => {
                    let m = monsters::spawn_monster(MonsterName::JawWorm, ascension, rng);
                    entities.push(Some(Entity { kind: EntityKind::Monster(m) }));
                }
                1 => {
                    let m = monsters::spawn_monster(MonsterName::Cultist, ascension, rng);
                    entities.push(Some(Entity { kind: EntityKind::Monster(m) }));
                }
                2 => {
                    let m1 = monsters::spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    let m2 = monsters::spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    entities.push(Some(Entity { kind: EntityKind::Monster(m1) }));
                    entities.push(Some(Entity { kind: EntityKind::Monster(m2) }));
                }
                _ => unreachable!(),
            };
            ProcessEffectResult::Continue {
                top: vec![Effect::CombatStart],
                bot: Vec::new(),
            }
        }
        RoomType::RestSite => ProcessEffectResult::Pass,
    }
}
