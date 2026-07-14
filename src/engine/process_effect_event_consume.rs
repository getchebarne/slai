use crate::game::GameState;

pub fn process_effect_event_consume(id_source: Option<usize>, state: &mut GameState) {
    let id_event = id_source.expect("EventConsume requires id_source");
    state.entities[id_event].event_consumed = true;

    // Entry-rolled picks die with the event
    state.id_event_picks.clear();
    state.event_gold_rolled = 0;
    state.event_rolls.clear();
}
