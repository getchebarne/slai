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
    let shiv = get_card(CardName::Shiv, false);

    for _ in 0..count {
        let card_id = EntityId(entities.len() as u32);
        entities.push(Entity {
            kind: EntityKind::Card(shiv),
        });

        if hand.len() < MAX_SIZE_HAND {
            hand.push(card_id)
        } else {
            disc_pile.push(card_id)
        }
    }

    ProcessEffectResult::Continue
}
