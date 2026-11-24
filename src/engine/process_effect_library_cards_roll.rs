use rand::Rng;

use crate::cards::get_card;
use crate::cards::get_card_template;
use crate::consts::LIBRARY_CARD_COUNT;
use crate::game::GameState;
use crate::relics::egg_upgrades_kind;
use crate::utils::LIBRARY_ROLL_POLICY;
use crate::utils::card_pool;
use crate::utils::push_entity;
use crate::utils::roll_card_rarity;

pub fn process_effect_library_cards_roll(state: &mut GameState) {
    assert!(state.event.active, "LibraryCardsRoll outside an event");

    // Get current Card reward roll offset
    let offset = state.entities[state.id_character].character_reward_roll_offset;

    // Initialize buffer
    let mut card_names_rolled = Vec::with_capacity(LIBRARY_CARD_COUNT);
    while card_names_rolled.len() < LIBRARY_CARD_COUNT {
        // Roll rarity
        let rarity = roll_card_rarity(
            &mut state.rng,
            offset,
            &LIBRARY_ROLL_POLICY,
            &state.id_relics,
        );
        let pool = card_pool(rarity, false);

        // Roll Card
        let card_name = pool[state.rng.random_range(0..pool.len())];

        if card_names_rolled.contains(&card_name) {
            // A duplicate re-rolls the rarity and the Card together
            continue;
        }
        card_names_rolled.push(card_name);

        // Eggs upgrade matching Cards at roll time, so the grid shows the truth
        let upgraded =
            egg_upgrades_kind(get_card_template(card_name, false).kind, &state.id_relics);
        let id_card = push_entity(&mut state.entities, get_card(card_name, upgraded));
        state.event.id_card_pile_event_roll.push(id_card);
    }
}
