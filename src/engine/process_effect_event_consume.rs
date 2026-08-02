use crate::game::GameState;
use crate::types::Mode;
use crate::utils::mode_top_mut;

pub fn process_effect_event_consume(state: &mut GameState) {
    let Mode::Event { consumed, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("EventConsume outside Event mode")
    };
    *consumed = true;
}
