use crate::game::GameState;

pub fn process_effect_relic_reward_remove_one(state: &mut GameState) {
    if !state.reward.id_relics.is_empty() {
        state.reward.id_relics.remove(0);
    }
}
