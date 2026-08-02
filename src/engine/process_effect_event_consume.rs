use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_event_consume(state: &mut GameState) {
    let Some(Mode::Event { consumed, .. }) = state.mode_stack.last_mut() else {
        unreachable!("EventConsume outside Event mode")
    };
    *consumed = true;
}
