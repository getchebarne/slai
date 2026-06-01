use crate::game::GameState;
use crate::modifier::modifier_set_not_new;

pub fn process_effect_modifier_set_not_new(state: &mut GameState) {
    let id_character = state.id_character;
    modifier_set_not_new(&mut state.entities[id_character].modifiers);
    let id_monsters = state.id_monsters;
    for id_monster in id_monsters.iter().flatten().copied() {
        modifier_set_not_new(&mut state.entities[id_monster].modifiers);
    }
}
