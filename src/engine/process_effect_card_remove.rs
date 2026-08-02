use crate::game::GameState;
use crate::types::Mode;
use crate::utils::mode_top_mut;

pub fn process_effect_card_remove(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat { id_hand, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("process_effect_card_remove outside Combat mode")
    };
    let id_card = id_target.expect("CardRemove requires id_target");
    if let Some(pos) = id_hand.iter().position(|&v| v == id_card) {
        id_hand.remove(pos);
    }
}
