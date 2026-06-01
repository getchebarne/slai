use crate::game::GameState;
use crate::modifier::modifier_tick;

pub fn process_effect_modifier_tick(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("ModifierTick requires id_target");
    modifier_tick(&mut state.entities[id_target].modifiers);
}
