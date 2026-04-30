use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::engine::process_effect_card_move_to_discard::process_effect_card_move_to_discard;
use crate::entity::Entity;

pub fn process_effect_card_discard_end_of_turn(
    id_target: usize,
    entities: &mut [Entity],
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if entities[id_target].card_retain {
        entities[id_target].card_retain = false;
        return DispatchResult::Continue;
    }
    if entities[id_target].card_ethereal {
        queue.push_front(Effect {
            kind: EffectKind::CardExhaust,
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        return DispatchResult::Continue;
    }
    // End-of-turn discard does not count as an explicit discard (no
    // `this_turn_discards`` increment)
    process_effect_card_move_to_discard(id_target, id_hand, id_pile_discard)
}
