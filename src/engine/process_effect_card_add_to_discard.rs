use crate::cards::get_card;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardName;

// Spawn `count` fresh copies of the named card and append them to the
// player's discard pile. Used by monster moves that inject status cards
// (Acid/Spike Slime Medium → Slimed; Sentry → Dazed).
pub fn process_effect_card_add_to_discard(
    card_name: CardName,
    count: u8,
    entities: &mut Vec<Entity>,
    id_pile_discard: &mut Vec<usize>,
) -> DispatchResult {
    if count == 0 {
        return DispatchResult::Continue;
    }
    for _ in 0..count {
        let template = get_card(card_name, false);
        let id_card = entities.len();
        entities.push(template);
        id_pile_discard.push(id_card);
    }
    DispatchResult::Continue
}
