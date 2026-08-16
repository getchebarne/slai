use crate::game::GameState;
use crate::monsters::get_next_move;
use crate::monsters::hexaghost;
use crate::monsters::is_cycle_boundary;
use crate::monsters::push_move_history;
use crate::types::Combat;
use crate::types::MonsterName;

pub fn process_effect_move_update(
    id_target: Option<usize>,
    state: &mut GameState,
    move_override: Option<usize>,
) {
    assert!(
        state.combat.active,
        "process_effect_move_update outside the Combat frame"
    );
    let Combat { id_monsters, .. } = &mut state.combat;
    let id_target = id_target.expect("MoveUpdate requires id_target");

    // Corpses don't roll: a mid-phase death leaves this queued effect dangling
    if state.entities[id_target].dead {
        return;
    }
    let character_health = state.entities[state.id_character].vitals.health;

    // A forced move (Split, wake-up) skips the AI and its RNG draw
    let move_next = match move_override {
        Some(idx) => idx,
        None => get_next_move(
            &state.entities,
            id_target,
            &id_monsters,
            state.ascension,
            &mut state.rng,
        ),
    };

    let entity = &mut state.entities[id_target];
    entity.monster_move_current = Some(move_next);

    // Divider damage locks in at selection; later HP changes don't move it
    entity.monster_move_damage_override = (entity.monster_name == MonsterName::Hexaghost
        && move_next == hexaghost::IDX_MOVE_DIVIDER)
        .then(|| character_health / 12 + 1);

    let move_idx = move_next as u8;
    push_move_history(entity, move_idx);

    if is_cycle_boundary(entity.monster_name, move_idx) {
        entity.monster_cycle_count += 1;
    }
}
