use rand::Rng;

use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::monsters::{self, Monster};
use crate::state::Map;
use crate::types::{EntityId, MonsterName, RoomType};

pub fn process_effect_room_enter(
    map: &Map,
    ascension: u8,
    monsters: &mut Vec<Monster>,
    next_entity_id: &mut u32,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    let room = map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => {
            let id = EntityId(*next_entity_id);
            *next_entity_id += 1;
            *monsters = vec![monsters::spawn_monster(
                id,
                MonsterName::TheGuardian,
                ascension,
                rng,
            )];
            ProcessEffectResult::Continue {
                top: vec![Effect::CombatStart],
                bot: Vec::new(),
            }
        }
        RoomType::CombatMonster => {
            let encounter: u8 = rng.random_range(0..3);
            *monsters = match encounter {
                0 => {
                    let id = EntityId(*next_entity_id);
                    *next_entity_id += 1;
                    vec![monsters::spawn_monster(id, MonsterName::JawWorm, ascension, rng)]
                }
                1 => {
                    let id = EntityId(*next_entity_id);
                    *next_entity_id += 1;
                    vec![monsters::spawn_monster(id, MonsterName::Cultist, ascension, rng)]
                }
                2 => {
                    let id_a = EntityId(*next_entity_id);
                    *next_entity_id += 1;
                    let id_b = EntityId(*next_entity_id);
                    *next_entity_id += 1;
                    vec![
                        monsters::spawn_monster(id_a, MonsterName::FungiBeast, ascension, rng),
                        monsters::spawn_monster(id_b, MonsterName::FungiBeast, ascension, rng),
                    ]
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
