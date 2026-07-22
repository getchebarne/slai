use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_event_consume(state: &mut GameState) {
    let Mode::Event { consumed, .. } = &mut state.mode else {
        unreachable!("EventConsume outside Event mode")
    };
    *consumed = true;
}
