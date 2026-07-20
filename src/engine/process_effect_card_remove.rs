use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_card_remove(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_card_remove outside Combat mode")
    };
    let id_card = id_target.expect("CardRemove requires id_target");
    if let Some(pos) = combat.id_hand.iter().position(|&v| v == id_card) {
        combat.id_hand.remove(pos);
    }
}
