use crate::game::GameState;

pub fn process_effect_relic_reward_remove_one(state: &mut GameState) {
    if !state.reward.id_relics.is_empty() {
        let id_relic = state.reward.id_relics.remove(0);

        // A removed Relic takes its linked Sapphire Key with it
        if state.reward.id_relic_linked == Some(id_relic) {
            state.reward.id_relic_linked = None;
        }
    }
}
