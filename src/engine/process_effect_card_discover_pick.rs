use rand::Rng;

use crate::cards::get_random_cards_of_kind;
use crate::entity::Entity;
use crate::types::CardKind;
use crate::types::Phase;

pub fn process_effect_card_discover_pick(
    kind: CardKind,
    count: u8,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> Option<Phase> {
    let card_picks = get_random_cards_of_kind(rng, kind, count as usize);
    let mut id_cards: Vec<usize> = Vec::with_capacity(card_picks.len());
    for card_pick in card_picks {
        let id = entities.len();
        entities.push(card_pick);
        id_cards.push(id);
    }
    Some(Phase::CombatAwaitDiscover { id_cards })
}
