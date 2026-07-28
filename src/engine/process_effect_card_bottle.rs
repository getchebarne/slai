use crate::game::GameState;

// Bottled cards join innate cards at the top of the opening draw pile
pub fn process_effect_card_bottle(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardBottle requires id_target");
    state.entities[id_card].card_bottled = true;
}
