use crate::game::GameState;

// Move-after-play to discard; does NOT count as discard nor fire Reflex/Tactician
pub fn process_effect_card_move_to_discard(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardMoveToDiscard requires id_target");
    if let Some(pos) = state.id_hand.iter().position(|&v| v == id_target) {
        state.id_hand.remove(pos);
    }
    state.id_pile_discard.push(id_target);
}
