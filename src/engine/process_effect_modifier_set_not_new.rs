use crate::game::GameState;
use crate::modifier::modifier_set_not_new;
use crate::types::Mode;
use crate::utils::mode_top_mut;

pub fn process_effect_modifier_set_not_new(state: &mut GameState) {
    let Mode::Combat { id_monsters, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("process_effect_modifier_set_not_new outside Combat mode")
    };
    let id_character = state.id_character;
    modifier_set_not_new(&mut state.entities[id_character].modifiers);
    for id_monster in id_monsters.iter().flatten().copied() {
        modifier_set_not_new(&mut state.entities[id_monster].modifiers);
    }
}
