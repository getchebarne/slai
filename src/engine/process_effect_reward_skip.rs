use crate::game::GameState;

pub fn process_effect_reward_skip(state: &mut GameState) {
    state.reward_id_cards.clear();
    state.reward_id_relic = None;
    state.reward_id_potion = None;
    state.reward_gold = None;
}
