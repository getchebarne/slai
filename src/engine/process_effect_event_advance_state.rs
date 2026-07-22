use crate::events::EventPayload;
use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_event_advance_state(state: &mut GameState, delta: i8) {
    let Mode::Event { payload, .. } = &mut state.mode else {
        unreachable!("EventAdvanceState outside Event mode")
    };
    let value = match payload {
        EventPayload::GoldenIdol { stage } => stage,
        EventPayload::ScrapOoze { attempts } => attempts,
        payload => unreachable!("EventAdvanceState on stateless event: {payload:?}"),
    };
    *value = (*value as i16 + delta as i16).clamp(0, u8::MAX as i16) as u8;
}
