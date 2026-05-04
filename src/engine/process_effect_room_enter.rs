use std::collections::VecDeque;

use rand::Rng;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH};
use crate::effect::{Effect, EffectKind};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::game::Location;
use crate::map::active_room_kind;
use crate::monsters::encounter::{
    generate_act1_elites, generate_act1_strong_only, spawn_encounter,
};
use crate::types::{MonsterEncounter, RoomKind};

pub fn process_effect_room_enter(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    monster_count: &mut u8,
    monster_list: &mut Vec<MonsterEncounter>,
    elite_monster_list: &mut Vec<MonsterEncounter>,
    boss_list: &mut Vec<MonsterEncounter>,
    rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    *monster_count = 0;

    let room = active_room_kind(id_rooms, location, entities).unwrap();
    let mut effects: Vec<Effect> = Vec::new();
    match room {
        RoomKind::CombatBoss => {
            // boss_list[0] is the actual fight; remove it on entry to match
            // AbstractDungeon's pop-front-on-completion (the run ends right
            // after, so timing is observationally equivalent).
            let encounter = boss_list.remove(0);
            spawn_encounter(encounter, rng, &mut effects);
        }
        RoomKind::CombatMonster => {
            if monster_list.is_empty() {
                generate_act1_strong_only(monster_list, rng);
            }
            let encounter = monster_list.remove(0);
            spawn_encounter(encounter, rng, &mut effects);
        }
        RoomKind::CombatElite => {
            if elite_monster_list.is_empty() {
                generate_act1_elites(elite_monster_list, rng);
            }
            let encounter = elite_monster_list.remove(0);
            spawn_encounter(encounter, rng, &mut effects);
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
