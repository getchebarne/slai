use crate::consts::MAX_GOLD;
use crate::game::GameState;

pub fn process_effect_gold_gain(state: &mut GameState, amount: u16) {
    let character = &mut state.entities[state.id_character];
    character.character_gold = character
        .character_gold
        .saturating_add(amount)
        .min(MAX_GOLD);
}
