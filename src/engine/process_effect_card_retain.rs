use crate::game::GameState;

pub fn process_effect_card_retain(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardRetain requires id_target");
    state.entities[id_target].card_retain = true;
}
