use crate::entity::push_move_history;
use crate::game::GameState;
use crate::monsters::get_next_move;
use crate::monsters::hexaghost;
use crate::monsters::is_cycle_boundary;
use crate::types::MonsterName;

pub fn process_effect_move_update(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("MoveUpdate requires id_target");
    let ascension_level = state.ascension;
    let id_monsters = state.id_monsters;
    let character_health = state.entities[state.id_character].vitals.health;

    let entity = &mut state.entities[id_target];
    let move_next = get_next_move(
        entity,
        id_target,
        &id_monsters,
        ascension_level,
        &mut state.rng,
    );

    entity.monster_move_current = Some(move_next);

    // Divider damage locks in at selection; later HP changes don't move it
    if entity.monster_name == MonsterName::Hexaghost && move_next == hexaghost::IDX_MOVE_DIVIDER {
        entity.monster_divider_damage = character_health / 12 + 1;
    }

    let move_idx = move_next as u8;
    push_move_history(entity, move_idx);

    if is_cycle_boundary(entity.monster_name, move_idx) {
        entity.monster_cycle_count += 1;
    }
}
