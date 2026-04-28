use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

// Finisher: deal `damage` to the target N times, where N is the number
// of attacks played this turn EXCLUDING Finisher itself. The card_play
// handler increments `this_turn_attacks_played` before fanning out the
// card's effects, so the counter at handler time includes Finisher — we
// subtract 1. If Finisher is the first attack of the turn, n = 0 → no hits,
// 0 damage
pub fn process_effect_finisher_damage(
    this_turn_attacks_played: u8,
    id_source: Option<usize>,
    id_target: usize,
    damage: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let num_attacks = this_turn_attacks_played.saturating_sub(1);
    for _ in 0..num_attacks {
        queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: damage },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
    DispatchResult::Continue
}
