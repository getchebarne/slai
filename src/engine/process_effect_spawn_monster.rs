use std::collections::VecDeque;

use rand::Rng;

use crate::consts::MAX_MONSTERS;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::monsters::spawn_monster;
use crate::types::MonsterName;

// TODO: generalize besides Slimes
pub fn process_effect_spawn_monster(
    name: MonsterName,
    id_source: usize,
    ascension_level: u8,
    entities: &mut Vec<Entity>,
    id_monsters: &mut [usize; MAX_MONSTERS],
    monster_count: &mut u8,
    rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    assert!(
        (*monster_count as usize) < MAX_MONSTERS,
        "SpawnMonster would overflow id_monsters: monster_count={} MAX={}",
        *monster_count,
        MAX_MONSTERS,
    );

    let health_parent = entities[id_source].vitals.health;
    let mut monster_child = spawn_monster(name, ascension_level, rng);

    // Slime split: spawned medium inherits the L's current HP as max health
    monster_child.vitals.health = health_parent;
    monster_child.vitals.health_max = health_parent;

    let id_child = entities.len();
    entities.push(monster_child);
    id_monsters[*monster_count as usize] = id_child;
    *monster_count += 1;

    // Queue a MoveUpdate so the spawned monster has an intent visible on the
    // next view rebuild and ready for its first turn
    queue.push_front(Effect {
        kind: EffectKind::MoveUpdate,
        id_source: None,
        target: Target::Direct(Some(id_child)),
    });

    DispatchResult::Continue
}
