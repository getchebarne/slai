use crate::game::GameState;

pub fn process_effect_card_purge(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardPurge requires id_target");
    if let Some(pos) = state.id_deck.iter().position(|&v| v == id_card) {
        state.id_deck.remove(pos);
    }
}
