use crate::game::GameState;
use crate::types::Mode;

// Move-after-play to discard; does NOT count as discard nor fire Reflex/Tactician
pub fn process_effect_card_move_to_discard(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat {
        id_hand,
        id_pile_discard,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_card_move_to_discard outside Combat mode")
    };
    let id_target = id_target.expect("CardMoveToDiscard requires id_target");
    if let Some(pos) = id_hand.iter().position(|&v| v == id_target) {
        id_hand.remove(pos);
    }
    id_pile_discard.push(id_target);
}
