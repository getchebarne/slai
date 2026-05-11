use std::collections::VecDeque;

use rand::Rng;

use crate::consts::MAX_MONSTERS;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::monsters::spawn_monster;
use crate::types::MonsterName;

pub fn process_effect_monster_spawn(
    name: MonsterName,
    id_source: Option<usize>,
    ascension_level: u8,
    entities: &mut Vec<Entity>,
    id_monsters: &mut [usize; MAX_MONSTERS],
    monster_count: &mut u8,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let mut monster_child = spawn_monster(name, ascension_level, rng);

    // Slime split: spawned child inherits the parent's current HP as max health.
    // Only the three splitting slimes use this path; gate on the source's
    // monster_name so a future non-slime caller passing a source doesn't
    // silently inherit HP
    if let Some(id) = id_source {
        let parent = &entities[id];
        assert!(
            matches!(
                parent.monster_name,
                MonsterName::SlimeAcidLarge
                    | MonsterName::SlimeSpikeLarge
                    | MonsterName::SlimeBoss
            ),
            "MonsterSpawn id_source must be a splitting slime, got {:?}",
            parent.monster_name,
        );
        let health_parent = parent.vitals.health;
        monster_child.vitals.health = health_parent;
        monster_child.vitals.health_max = health_parent;
    }

    let id_child = entities.len();
    entities.push(monster_child);

    // `monster_count` is the high-water mark of slots used this combat;
    // it never decrements on death (Death just marks `dead = true`). When
    // a splitting slime cascades (LargeSlime → 2 mediums → 4 smalls), the
    // count would walk past MAX_MONSTERS even though never more than 4 are
    // simultaneously alive. Reuse dead slots so the count tracks the
    // actual occupancy ceiling.
    let mut reused_slot: Option<usize> = None;
    for i in 0..(*monster_count as usize) {
        if entities[id_monsters[i]].dead {
            reused_slot = Some(i);
            break;
        }
    }

    let slot = match reused_slot {
        Some(s) => s,
        None => {
            assert!(
                (*monster_count as usize) < MAX_MONSTERS,
                "MonsterSpawn would overflow id_monsters with no dead slot to \
                 reuse: monster_count={} MAX={}",
                *monster_count,
                MAX_MONSTERS,
            );
            let s = *monster_count as usize;
            *monster_count += 1;
            s
        }
    };
    id_monsters[slot] = id_child;

    // Queue a MoveUpdate so the spawned monster has an intent visible on the
    // next view rebuild and ready for its first turn
    effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate,
        id_source: None,
        target: Target::Direct(Some(id_child)),
    });

    DispatchResult::Continue
}
