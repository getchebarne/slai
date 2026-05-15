use rand::Rng;

use crate::cards::get_random_cards_of_kind;
use crate::engine::DispatchResult;
use crate::engine::enter_discover;
use crate::entity::Entity;
use crate::types::CardKind;
use crate::types::Phase;

pub fn process_effect_card_discover_pick(
    kind: CardKind,
    count: u8,
    phase: &mut Phase,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    let card_picks = get_random_cards_of_kind(rng, kind, count as usize);
    let mut rolled_ids: Vec<usize> = Vec::with_capacity(card_picks.len());
    for card_pick in card_picks {
        let id = entities.len();
        entities.push(card_pick);
        rolled_ids.push(id);
    }
    let Phase::CombatAwaitDiscover { id_cards } = enter_discover(phase) else { unreachable!() };
    id_cards.clear();
    id_cards.extend(rolled_ids);
    DispatchResult::Continue
}
