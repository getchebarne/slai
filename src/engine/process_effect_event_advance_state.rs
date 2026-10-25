use crate::game::GameState;
use crate::types::EventName;
use crate::types::Focus;
use crate::utils::context_focus;

pub fn process_effect_event_advance_state(state: &mut GameState, delta: i8) {
    assert!(
        context_focus(state) == Focus::Event,
        "EventAdvanceState outside the Event context"
    );
    assert!(
        matches!(
            state.event.name,
            EventName::GoldenIdol
                | EventName::Colosseum
                | EventName::ScrapOoze
                | EventName::CursedTome
        ),
        "EventAdvanceState on stateless event: {:?}",
        state.event.name
    );
    let stage = &mut state.event.stage;
    *stage = (*stage as i16 + delta as i16).clamp(0, u8::MAX as i16) as u8;
}
