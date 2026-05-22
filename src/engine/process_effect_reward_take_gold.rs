use crate::game::GameState;

pub fn process_effect_reward_take_gold(state: &mut GameState) {
    if let Some(amount) = state.reward_gold.take() {
        let character = &mut state.entities[state.id_character];
        character.character_gold = character.character_gold.saturating_add(amount);
    }
}
