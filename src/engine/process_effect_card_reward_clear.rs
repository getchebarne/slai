use crate::game::GameState;

pub fn process_effect_card_reward_clear(state: &mut GameState) {
    state.reward_id_cards.clear();
}
