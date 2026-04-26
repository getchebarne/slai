use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

// Finisher: deal `damage_per` to the target N times, where N is the number
// of attacks played this turn EXCLUDING Finisher itself. The card_play
// handler increments `attacks_played_this_turn` before fanning out the
// card's effects, so the counter at handler time includes Finisher — we
// subtract 1 (matches StS DamagePerAttackPlayedAction's `--count`).
//
// If Finisher is the first attack of the turn, n = 0 → no hits, 0 damage.
pub fn process_effect_finisher_damage(
    attacks_played_this_turn: u8,
    id_source: Option<usize>,
    id_target: usize,
    damage_per: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let n = attacks_played_this_turn.saturating_sub(1);
    for _ in 0..n {
        queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: damage_per },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
    DispatchResult::Continue
}
