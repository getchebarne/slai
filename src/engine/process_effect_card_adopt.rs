use crate::game::GameState;

pub fn process_effect_card_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardAdopt requires id_target");
    state.id_deck.push(id_card);
}
