use crate::game::GameState;

pub fn process_effect_event_consume(state: &mut GameState) {
    assert!(state.event.active, "EventConsume outside an event");
    state.event.consumed = true;

    // A consumed Event's offers are dead
    state.event.id_card_pile_event_roll.clear();
    state.event.id_relic_event_roll.clear();
    state.event.id_potion_event_roll.clear();
}
