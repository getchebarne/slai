use crate::game::GameState;
use crate::modifier::modifier_set_not_new;
use crate::modifier::modifier_tick;

pub fn process_effect_modifier_tick(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("ModifierTick requires id_target");
    let modifiers = &mut state.entities[id_target].modifiers;

    // A Monster's Modifiers stop being new here, ending its first-round Ritual skip; only debuffs on the Character skip a round end
    if id_target != state.id_character {
        modifier_set_not_new(modifiers);
    }
    modifier_tick(modifiers);
}
