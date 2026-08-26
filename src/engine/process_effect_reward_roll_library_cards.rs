use crate::consts::LIBRARY_CARD_COUNT;
use crate::game::GameState;
use crate::types::reward_ensure;
use crate::utils::roll_card_rewards;

// The Library: one pick-a-Card bundle of 20 unique rarity-rolled Cards
pub fn process_effect_reward_roll_library_cards(state: &mut GameState) {
    let mut id_cards: Vec<usize> = Vec::with_capacity(LIBRARY_CARD_COUNT);
    roll_card_rewards(
        state.id_character,
        &mut state.entities,
        &mut state.rng,
        &mut id_cards,
        &state.id_relics,
        LIBRARY_CARD_COUNT,
        false,
    );

    reward_ensure(&mut state.reward);
    state.reward.id_cards.push(id_cards);
}
