use rand::Rng;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_COLORLESS_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_COLORLESS_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::consts::NEOW_CARD_COUNT;
use crate::consts::NEOW_UNCOMMON_CHANCE;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::reward_ensure;
use crate::utils::push_entity;

// Neow's Card offers: 33% Uncommon else Common, never Rare unless forced
pub fn process_effect_reward_roll_neow_cards(
    state: &mut GameState,
    colorless: bool,
    rare_only: bool,
) {
    // Initialize containers
    let mut id_cards: Vec<usize> = Vec::with_capacity(NEOW_CARD_COUNT);
    let mut card_names_rolled: [CardName; NEOW_CARD_COUNT] = [CardName::Strike; NEOW_CARD_COUNT];

    for idx in 0..NEOW_CARD_COUNT {
        // Pick pool
        let card_pool: &[CardName] = match (colorless, rare_only) {
            (true, true) => POOL_RARE_COLORLESS_CARD,
            (true, false) => POOL_UNCOMMON_COLORLESS_CARD,
            (false, true) => POOL_RARE_GREEN_CARD,
            (false, false) => {
                if state.rng.random_bool(NEOW_UNCOMMON_CHANCE) {
                    POOL_UNCOMMON_GREEN_CARD
                } else {
                    POOL_COMMON_GREEN_CARD
                }
            }
        };

        // Roll the Card's name — loops until the rolled Card hasn't been rolled already
        let mut card_name = card_pool[state.rng.random_range(0..card_pool.len())];
        while card_names_rolled[..idx].contains(&card_name) {
            card_name = card_pool[state.rng.random_range(0..card_pool.len())];
        }

        // Store the rolled Card's name and push its Entity to the `id_cards` collection
        card_names_rolled[idx] = card_name;
        id_cards.push(push_entity(&mut state.entities, get_card(card_name, false)));
    }

    reward_ensure(&mut state.reward);
    state.reward.id_cards.push(id_cards);
}
