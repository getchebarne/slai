use crate::game::GameState;

pub fn process_effect_gold_loss(state: &mut GameState, amount: u16) {
    let character = &mut state.entities[state.id_character];
    character.character_gold = character.character_gold.saturating_sub(amount);
}
