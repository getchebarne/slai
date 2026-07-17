use crate::game::GameState;

pub fn process_effect_event_consume(id_source: Option<usize>, state: &mut GameState) {
    let id_event = id_source.expect("EventConsume requires id_source");
    state.entities[id_event].event_consumed = true;

    // Entry-rolled picks die with the event (scalar rolls live on the entity itself)
    state.id_event_picks.clear();
}
