use rand::Rng;

use crate::cards::get_random_cards_of_kind;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardKind;

pub fn process_effect_card_discover_pick(
    kind: CardKind,
    count: u8,
    id_card_discover: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    // Clear
    id_card_discover.clear();

    // Roll picks
    let card_picks = get_random_cards_of_kind(rng, kind, count as usize);

    // Push entities
    for card_pick in card_picks {
        let id = entities.len();
        entities.push(card_pick);
        id_card_discover.push(id);
    }
    DispatchResult::Continue
}
