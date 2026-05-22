use crate::game::GameState;

pub fn process_effect_reward_take_relic(state: &mut GameState) {
    if let Some(id) = state.reward_id_relic.take() {
        let name = state.entities[id].relic_name;
        state.id_relics[name as usize] = Some(id);
    }
}
