use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;

pub fn process_effect_card_remove(id_target: Option<usize>, state: &mut GameState) {
    let Frame::Combat { id_hand, .. } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("process_effect_card_remove outside the Combat frame")
    };
    let id_card = id_target.expect("CardRemove requires id_target");
    if let Some(pos) = id_hand.iter().position(|&v| v == id_card) {
        id_hand.remove(pos);
    }
}
