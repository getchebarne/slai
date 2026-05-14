use rand::Rng;

use crate::cards::get_card;
use crate::cards::get_random_cards_of_kind;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardKind;

pub fn process_effect_discover_pick(
    kind: CardKind,
    count: u8,
    id_card_picks: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    let names = get_random_cards_of_kind(rng, kind, count as usize);
    id_card_picks.clear();
    for name in names {
        let id = entities.len();
        entities.push(get_card(name, false));
        id_card_picks.push(id);
    }
    DispatchResult::Continue
}
