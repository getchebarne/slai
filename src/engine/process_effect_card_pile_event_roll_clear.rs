use crate::game::GameState;

// The event's rolled Cards leave it
pub fn process_effect_card_pile_event_roll_clear(state: &mut GameState) {
    assert!(
        state.event.active,
        "CardPileEventRollClear outside an event"
    );
    state.event.id_card_pile_event_roll.clear();
}
