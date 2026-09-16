use crate::game::GameState;

pub fn process_effect_card_retain(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardRetain requires id_target");
    let card = &mut state.entities[id_target];
    if !card.card_ethereal {
        card.card_retain = true;
    }
}
