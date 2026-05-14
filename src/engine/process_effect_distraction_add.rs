use rand::Rng;

use crate::cards::POOL_COMMON;
use crate::cards::POOL_RARE;
use crate::cards::POOL_UNCOMMON;
use crate::cards::get_card;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::entity::add_card_to_hand_or_discard;
use crate::types::CardKind;
use crate::types::CardName;

// Distraction: spawn a random Silent Skill (excluding Distraction itself) as
// a free-to-play-once card in hand
pub fn process_effect_distraction_add(
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    rng: &mut impl Rng,
) -> DispatchResult {
    // Build the candidate pool
    // Stack buffer big enough for the current pool
    let mut buf = [CardName::Strike; 64];
    let mut n = 0;
    for pool in [POOL_COMMON, POOL_UNCOMMON, POOL_RARE] {
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

    add_card_to_hand_or_discard(entities, id_hand, id_pile_discard, card);
    DispatchResult::Continue
}
