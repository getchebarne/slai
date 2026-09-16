use crate::effect::Effect;
use crate::game::GameState;
use crate::utils::flush_effects_from_buf_to_queue_front;

// The option's effects run in definition order, sourced to the option
pub fn process_effect_event_option_select(id_target: Option<usize>, state: &mut GameState) {
    let id_option = id_target.expect("EventOptionSelect requires id_target");
    let option = &state.entities[id_option];
    let effects = option.event_option_effects;
    let effects_len = option.event_option_effects_len as usize;
    state.effect_buf.clear();
    for effect in &effects[..effects_len] {
        state.effect_buf.push(Effect {
            id_source: Some(id_option),
            ..*effect
        });
    }
    flush_effects_from_buf_to_queue_front(state);
}
