use crate::utils::queue_room_select;
use crate::game::GameState;
use crate::types::Screen;

pub fn process_effect_event_end(id_source: Option<usize>, state: &mut GameState) {
    let id_event = id_source.expect("EventEnd requires id_source");
    state.entities[id_event].event_consumed = true;
    state.screen = Screen::Map;
    state.id_event = None;
    queue_room_select(state);
}
