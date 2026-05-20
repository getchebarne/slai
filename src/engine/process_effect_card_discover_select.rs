use rand::Rng;

use crate::cards::get_random_cards_of_kind_and_color;
use crate::entity::Entity;
use crate::types::CardColor;
use crate::types::CardKind;

// Rolls `count` random cards of the given kind/color. Returns the entity ids
// of the rolled cards. The caller (dispatcher) is responsible for storing
// them on CombatState::id_pick and queuing a CardDiscoverPick halt
pub fn process_effect_card_discover_select(
    kind: CardKind,
    color: CardColor,
    count: u8,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> Vec<usize> {
    let card_picks = get_random_cards_of_kind_and_color(kind, color, count as usize, rng);
    let mut id_cards: Vec<usize> = Vec::with_capacity(card_picks.len());
    for card_pick in card_picks {
        let id = entities.len();
        entities.push(card_pick);
        id_cards.push(id);
    }
    id_cards
}
