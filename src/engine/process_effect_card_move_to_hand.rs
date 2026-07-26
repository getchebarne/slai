use crate::consts::MAX_SIZE_HAND;
use crate::game::GameState;
use crate::types::Mode;

// Move from the draw pile to hand; not a draw (no on-draw hooks, no last-drawn update)
pub fn process_effect_card_move_to_hand(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat {
        id_hand,
        id_pile_draw,
        id_pile_discard,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_card_move_to_hand outside Combat mode")
    };
    let id_target = id_target.expect("CardMoveToHand requires id_target");
    if let Some(pos) = id_pile_draw.iter().position(|&v| v == id_target) {
        id_pile_draw.remove(pos);
    }
    // Full hand routes to discard (StS SkillFromDeckToHandAction behavior)
    if id_hand.len() < MAX_SIZE_HAND {
        id_hand.push(id_target);
    } else {
        id_pile_discard.push(id_target);
    }
}
