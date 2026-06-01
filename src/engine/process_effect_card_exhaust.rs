use crate::game::GameState;

pub fn process_effect_card_exhaust(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardExhaust requires id_target");
    if let Some(pos) = state.id_hand.iter().position(|&v| v == id_card) {
        state.id_hand.remove(pos);
    }
    state.id_pile_exhaust.push(id_card);
}
