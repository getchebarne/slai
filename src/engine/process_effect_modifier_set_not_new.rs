use crate::consts::MAX_MONSTERS;
use crate::game::GameState;
use crate::modifier::modifier_set_not_new;
use crate::utils::fill_alive_monster_ids;

pub fn process_effect_modifier_set_not_new(state: &mut GameState) {
    let mut buf_alive = [0usize; MAX_MONSTERS];
    let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
    let id_character = state.id_character;
    modifier_set_not_new(&mut state.entities[id_character].modifiers);
    for &id_monster in &buf_alive[..alive_n] {
        modifier_set_not_new(&mut state.entities[id_monster].modifiers);
    }
}
