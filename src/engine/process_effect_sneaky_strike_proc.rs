use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

// SneakyStrike bonus: gain `energy` energy if at least one card has been
// explicitly discarded this turn. The counter is maintained in
// `process_effect_card_discard` (NOT the move-after-play or end-of-turn
// variants — see CardMoveToDiscard).
pub fn process_effect_sneaky_strike_proc(
    discards_this_turn: u8,
    energy: u8,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if discards_this_turn == 0 {
        return DispatchResult::Continue;
    }
    queue.push_front(Effect {
        kind: EffectKind::EnergyGain { amount: energy },
        id_source: None,
        target: Target::Direct(None),
    });
    DispatchResult::Continue
}
