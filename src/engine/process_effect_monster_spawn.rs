use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::monsters::spawn_monster;
use crate::types::MonsterName;
use crate::utils::push_entity;

pub fn process_effect_monster_spawn(
    id_source: Option<usize>,
    state: &mut GameState,
    name: MonsterName,
) {
    // Create the monster `Entity`
    let mut monster = spawn_monster(name, state.ascension, &mut state.rng);

    // Slime split: child max HP = parent current HP; gated to splittable slimes only.
    // Non-monster sources (an event option stamps its event as id_source) spawn plain
    if let Some(id) = id_source
        && state.entities[id].kind == EntityKind::Monster
    {
        let parent = &state.entities[id];
        assert!(
            matches!(
                parent.monster_name,
                MonsterName::SlimeAcidLarge | MonsterName::SlimeSpikeLarge | MonsterName::SlimeBoss
            ),
            "MonsterSpawn id_source must be a splitting slime, got {:?}",
            parent.monster_name,
        );
        monster.vitals.health = parent.vitals.health;
        monster.vitals.health_max = parent.vitals.health;
    }

    // Push it
    let id_monster = push_entity(&mut state.entities, monster);

    // Place it in the first empty monster slot
    let idx = state
        .id_monsters
        .iter()
        .position(|s| s.is_none())
        .expect("MonsterSpawn would overflow id_monsters: no empty idx");
    state.id_monsters[idx] = Some(id_monster);

    // Queue an effect to update its move
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate {
            move_override: None,
        },
        id_source: None,
        target: Target::Direct(Some(id_monster)),
    });
}
