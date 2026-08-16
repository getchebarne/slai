use crate::game::GameState;
use crate::types::Combat;

pub fn process_effect_card_remove(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_remove outside the Combat frame"
    );
    let Combat { id_card_hand, .. } = &mut state.combat;
    let id_card = id_target.expect("CardRemove requires id_target");
    if let Some(pos) = id_card_hand.iter().position(|&id| id == id_card) {
        id_card_hand.remove(pos);
    }
}
