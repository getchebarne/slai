use crate::game::GameState;

pub fn process_effect_event_end(id_source: Option<usize>, state: &mut GameState) {
    let id_event = id_source.expect("EventEnd requires id_source");
    state.entities[id_event].event_consumed = true;
}
