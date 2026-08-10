use crate::events::EventKind;
use crate::game::GameState;
use crate::types::Mode;
use crate::utils::mode_top_mut;

pub fn process_effect_event_advance_state(state: &mut GameState, delta: i8) {
    let Mode::Event { kind, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("EventAdvanceState outside Event mode")
    };
    let value = match kind {
        EventKind::GoldenIdol { stage } => stage,
        EventKind::Colosseum { stage } => stage,
        EventKind::ScrapOoze { attempts } => attempts,
        kind => unreachable!("EventAdvanceState on stateless event: {kind:?}"),
    };
    *value = (*value as i16 + delta as i16).clamp(0, u8::MAX as i16) as u8;
}
