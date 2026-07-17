use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::spawn_monster;
use crate::types::MonsterName;
use crate::utils::push_entity;

pub fn process_effect_monster_spawn(state: &mut GameState, name: MonsterName) {
    // Create the monster `Entity`
    let monster = spawn_monster(name, state.ascension, &mut state.rng);

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
