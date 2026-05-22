use crate::game::GameState;

// Move-after-play: pushed by `process_effect_card_play` to send the
// just-played card to the discard pile. Same hand/discard mutation as
// `process_effect_card_discard`, but does NOT count as a "discard this turn"
// and does NOT fire Reflex/Tactician triggers
pub fn process_effect_card_move_to_discard(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardMoveToDiscard requires id_target");
    if let Some(pos) = state.id_hand.iter().position(|&v| v == id_target) {
        state.id_hand.remove(pos);
    }
    state.id_pile_discard.push(id_target);
}
