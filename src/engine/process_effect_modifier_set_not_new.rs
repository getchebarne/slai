use crate::game::GameState;
use crate::modifier::modifier_set_not_new;
use crate::types::Combat;

pub fn process_effect_modifier_set_not_new(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_modifier_set_not_new outside the Combat frame"
    );
    let Combat { id_monsters, .. } = &mut state.combat;
    let id_character = state.id_character;
    modifier_set_not_new(&mut state.entities[id_character].modifiers);
    for id_monster in id_monsters.iter().flatten().copied() {
        modifier_set_not_new(&mut state.entities[id_monster].modifiers);
    }
}
