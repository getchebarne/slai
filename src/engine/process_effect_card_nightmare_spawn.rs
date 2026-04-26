use crate::consts::MAX_SIZE_HAND;
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Drains the Nightmare-pending vec into hand at the next character TurnStart.
// Each pending Entity is a snapshot taken at NightmarePick time, so per-card
// state (GlassKnife damage decay, Setup free-to-play flag, etc.) is intact
// on the spawned copies.
pub fn process_effect_card_nightmare_spawn(
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    cards_nightmare: &mut Vec<Entity>,
) -> DispatchResult {
    for card in cards_nightmare.drain(..) {
        let id_card = entities.len();
        entities.push(card);
        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);
        } else {
            id_pile_discard.push(id_card);
        }
    }
    DispatchResult::Continue
}
