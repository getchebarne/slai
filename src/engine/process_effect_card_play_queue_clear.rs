use crate::game::GameState;

// Every waiting Card play is dropped and fires nothing: a draw-top Card stays in no pile, a replay never plays, and a Card waiting to play itself stays in the hand
pub fn process_effect_card_play_queue_clear(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_play_queue_clear outside the Combat frame"
    );
    state.card_play_queue.clear();
}
