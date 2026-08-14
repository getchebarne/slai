use crate::events::EventKind;
use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;

pub fn process_effect_event_advance_state(state: &mut GameState, delta: i8) {
    let Frame::Event { kind, .. } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("EventAdvanceState outside the Event frame")
    };
    let value = match kind {
        EventKind::GoldenIdol { stage } => stage,
        EventKind::Colosseum { stage } => stage,
        EventKind::ScrapOoze { attempts } => attempts,
        kind => unreachable!("EventAdvanceState on stateless event: {kind:?}"),
    };
    *value = (*value as i16 + delta as i16).clamp(0, u8::MAX as i16) as u8;
}
