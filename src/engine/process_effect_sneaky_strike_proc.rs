use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::DispatchResult;

// SneakyStrike bonus: gain `energy` energy if at least one card has been
// explicitly discarded this turn. The counter is increased in
// `process_effect_card_discard`
pub fn process_effect_sneaky_strike_proc(
    this_turn_discards: u8,
    energy: u8,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if this_turn_discards == 0 {
        return DispatchResult::Continue;
    }
    effect_queue.push_front(Effect {
        kind: EffectKind::EnergyGain { amount: energy },
        id_source: None,
        target: Target::Direct(None),
    });
    DispatchResult::Continue
}
