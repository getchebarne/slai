use crate::cards::get_card;
use crate::consts::MAX_SIZE_HAND;
use crate::engine::ProcessEffectResult;
use crate::state::{Entity, EntityKind};
use crate::types::{CardName, EntityId};

pub fn process_effect_add_shivs(
    count: u8,
    entities: &mut Vec<Entity>,
    hand: &mut Vec<EntityId>,
    disc_pile: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    // Get Shiv card definition
    let shiv = get_card(CardName::Shiv, false);

    // Create entities
    for _ in 0..count {
        // Get id
        let id_card = EntityId(entities.len() as u32);

        // Create and push entity
        entities.push(Entity {
            kind: EntityKind::Card(shiv),
        });

        // Add to hand or discard pile, depending on hand size
        if hand.len() < MAX_SIZE_HAND {
            hand.push(id_card)
        } else {
            disc_pile.push(id_card)
        }
    }

    // Continue
    ProcessEffectResult::Continue
}
