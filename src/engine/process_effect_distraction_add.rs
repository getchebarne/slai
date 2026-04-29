use rand::Rng;

use crate::cards::{REWARD_POOL_COMMON, REWARD_POOL_RARE, REWARD_POOL_UNCOMMON, get_card};
use crate::consts::MAX_SIZE_HAND;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::{CardKind, CardName};

// Distraction: spawn a random Silent Skill (excluding Distraction itself) as
// a free-to-play-once card in hand
pub fn process_effect_distraction_add(
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    rng: &mut impl Rng,
) -> DispatchResult {
    // Build the candidate pool
    // Stack buffer big enough for the current pool.
    let mut buf = [CardName::Strike; 64];
    let mut n = 0;
    for pool in [REWARD_POOL_COMMON, REWARD_POOL_UNCOMMON, REWARD_POOL_RARE] {
        for &name in pool {
            if name == CardName::Distraction {
                continue;
            }
            if get_card(name, false).card_kind != CardKind::Skill {
                continue;
            }
            buf[n] = name;
            n += 1;
        }
    }
    if n == 0 {
        return DispatchResult::Continue;
    }

    // Pick a random card
    let card_name = buf[rng.random_range(0..n)];
    let mut card = get_card(card_name, false);

    // Set free-to-play-once flag
    card.card_free_to_play_once = true;

    // Add card to entities buffer
    let id_card = entities.len();
    entities.push(card);

    // Add card to either the hand or the discard pile if the hand is full
    if id_hand.len() < MAX_SIZE_HAND {
        id_hand.push(id_card);
    } else {
        id_pile_discard.push(id_card);
    }

    // Continue
    DispatchResult::Continue
}
