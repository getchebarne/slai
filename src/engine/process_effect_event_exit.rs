use crate::game::GameState;
use crate::types::Screen;
use crate::utils::queue_room_select;

pub fn process_effect_event_exit(state: &mut GameState) {
    state.screen = Screen::Map;
    state.id_event = None;
    queue_room_select(state);
}
