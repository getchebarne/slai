use crate::events::EventKind;
use crate::game::GameState;
use crate::types::Focus;
use crate::utils::context_focus;

pub fn process_effect_event_advance_state(state: &mut GameState, delta: i8) {
    assert!(
        context_focus(state) == Focus::Event,
        "EventAdvanceState outside the Event context"
    );
    let value = match &mut state.event.event_kind {
        EventKind::GoldenIdol { stage } => stage,
        EventKind::Colosseum { stage } => stage,
        EventKind::ScrapOoze { attempts } => attempts,
        EventKind::CursedTome { stage } => stage,
        event_kind => unreachable!("EventAdvanceState on stateless event: {event_kind:?}"),
    };
    *value = (*value as i16 + delta as i16).clamp(0, u8::MAX as i16) as u8;
}
