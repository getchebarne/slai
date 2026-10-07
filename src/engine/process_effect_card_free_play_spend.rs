use crate::game::GameState;

// A Card's free play is spent without a play: a Confusion draw, or an ethereal Card's end-of-turn exhaust
pub fn process_effect_card_free_play_spend(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardFreePlaySpend requires id_target");
    state.entities[id_card].card_free_to_play_once = false;
}
