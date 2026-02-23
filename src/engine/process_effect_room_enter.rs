use rand::Rng;

use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::monsters::{self, Monster};
use crate::state::Map;
use crate::types::{MonsterName, RoomType};

pub fn process_effect_room_enter(
    map: &Map,
    ascension: u8,
    monsters: &mut Vec<Monster>,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    let room = map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => {
            // TODO: other bosses
            *monsters = vec![monsters::spawn_monster(
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
            let roll: u8 = rng.random_range(0..3);
            *monsters = match roll {
                0 => vec![monsters::spawn_monster(
                    MonsterName::JawWorm,
                    ascension,
                    rng,
                )],
                1 => vec![monsters::spawn_monster(
                    MonsterName::Cultist,
                    ascension,
                    rng,
                )],
                2 => vec![
                    monsters::spawn_monster(MonsterName::FungiBeast, ascension, rng),
                    monsters::spawn_monster(MonsterName::FungiBeast, ascension, rng),
                ],
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
