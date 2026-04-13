use crate::cards::get_card;
use crate::consts::MAX_SIZE_HAND;
use crate::engine::ProcessEffectResult;
use crate::state::{Entity, EntityKind};
use crate::types::CardName;

pub fn process_effect_add_shivs(
    count: u8,
    entities: &mut Vec<Entity>,
    hand: &mut Vec<usize>,
    discard_pile: &mut Vec<usize>,
) -> ProcessEffectResult {
    let shiv = get_card(CardName::Shiv, false);

    // Create shiv entities, overflow to discard pile if hand is full
    for _ in 0..count {
        let id_card = entities.len();
        entities.push(Entity {
            kind: EntityKind::Card(shiv),
        });

        if hand.len() < MAX_SIZE_HAND {
            hand.push(id_card)
        } else {
            discard_pile.push(id_card)
        }
    }

    // Continue
    ProcessEffectResult::Continue
}
