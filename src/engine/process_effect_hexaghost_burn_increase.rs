use std::collections::VecDeque;

use crate::cards::get_card;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardName;

// Upgrade every Burn in draw and discard piles, then add `count` upgraded
// Burns to discard
pub fn process_effect_hexaghost_burn_increase(
    count: u8,
    entities: &mut Vec<Entity>,
    id_pile_draw: &[usize],
    id_pile_discard: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let burn_upgraded = get_card(CardName::Burn, true);
    for &id_card in id_pile_draw.iter().chain(id_pile_discard.iter()) {
        if entities[id_card].card_name == CardName::Burn && !entities[id_card].card_upgraded {
            entities[id_card] = burn_upgraded;
        }
    }

    if count > 0 {
        effect_queue.push_front(Effect {
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Burn,
                count,
                upgraded: true, // upgraded
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    DispatchResult::Continue
}
