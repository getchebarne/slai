use rand::Rng;

use crate::cards::get_card;
use crate::consts::NEOW_UNCOMMON_CHANCE;
use crate::consts::NUM_NEOW_CARDS;
use crate::game::GameState;
use crate::types::CardRarity;
use crate::utils::card_pool;
use crate::utils::push_entity;

// Neow's card offer rolls 3 Cards into the event's roll: Uncommon 33% and Common otherwise, or all Rare; a colorless offer turns Common into Uncommon
pub fn process_effect_neow_cards_roll(state: &mut GameState, colorless: bool, rare_only: bool) {
    assert!(state.event.active, "NeowCardsRoll outside an event");
    let mut card_names_rolled = Vec::with_capacity(NUM_NEOW_CARDS);
    for _ in 0..NUM_NEOW_CARDS {
        let rarity = if rare_only {
            CardRarity::Rare
        } else {
            let roll = state.rng.random_range(0i32..=99);
            if colorless || roll < NEOW_UNCOMMON_CHANCE {
                CardRarity::Uncommon
            } else {
                CardRarity::Common
            }
        };
        let pool = card_pool(rarity, colorless);

        // A duplicate re-rolls within the rarity
        let mut card_name = pool[state.rng.random_range(0..pool.len())];
        while card_names_rolled.contains(&card_name) {
            card_name = pool[state.rng.random_range(0..pool.len())];
        }
        card_names_rolled.push(card_name);
        let id_card = push_entity(&mut state.entities, get_card(card_name, false));
        state.event.id_card_pile_event_roll.push(id_card);
    }
}
