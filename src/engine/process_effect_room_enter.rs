use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::consts::{MAP_HEIGHT, MAP_WIDTH};
use crate::map::active_room_kind;
use crate::monsters::spawn_monster;
use crate::state::Location;
use crate::types::{MonsterName, RoomKind};

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
            let m = spawn_monster(MonsterName::TheGuardian, ascension, rng);
            push_monster(m, entities, id_monsters, monster_count);
            queue.push_front(Effect {
                kind: EffectKind::CombatStart,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::CombatMonster => {
            let encounter: u8 = rng.random_range(0..3);
            match encounter {
                0 => {
                    let m = spawn_monster(MonsterName::JawWorm, ascension, rng);
                    push_monster(m, entities, id_monsters, monster_count);
                }
                1 => {
                    let m = spawn_monster(MonsterName::Cultist, ascension, rng);
                    push_monster(m, entities, id_monsters, monster_count);
                }
                2 => {
                    let m1 = spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    let m2 = spawn_monster(MonsterName::FungiBeast, ascension, rng);
                    push_monster(m1, entities, id_monsters, monster_count);
                    push_monster(m2, entities, id_monsters, monster_count);
                }
                _ => unreachable!(),
            };
            queue.push_front(Effect {
                kind: EffectKind::CombatStart,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite => {
            queue.push_front(Effect::direct(EffectKind::AwaitRestSiteAction, None, None));
        }
    }
    DispatchResult::Continue
}
