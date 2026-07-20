use crate::events::EventPayload;
use crate::game::GameState;

pub fn process_effect_event_advance_state(state: &mut GameState, delta: i8) {
    let event = state
        .event
        .as_mut()
        .expect("EventAdvanceState without an active event");
    let bump = |value: u8| ((value as i16 + delta as i16).clamp(0, u8::MAX as i16)) as u8;
    match &mut event.payload {
        EventPayload::GoldenIdol { stage } => *stage = bump(*stage),
        EventPayload::ScrapOoze { attempts } => *attempts = bump(*attempts),
        payload => unreachable!("EventAdvanceState on stateless event: {payload:?}"),
    }
}
