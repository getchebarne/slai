use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;

pub fn process_effect_event_consume(state: &mut GameState) {
    let Frame::Event { consumed, .. } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("EventConsume outside the Event frame")
    };
    *consumed = true;
}
