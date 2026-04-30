use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Hexaghost Divider: 6 hits of (player_current_hp / 12) + 1 each.
// Java locks the value at Activate time; slai computes at Divider fire time
// (functionally equivalent in normal play — no source damages the player
// between Activate and Divider).
pub fn process_effect_hexaghost_divider(
    hits: u8,
    id_source: Option<usize>,
    id_character: usize,
    entities: &[Entity],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let hp = entities[id_character].vitals.health;
    let dmg: u16 = hp / 12 + 1;

    for _ in 0..hits {
        queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: dmg },
            id_source,
            target: Target::Direct(Some(id_character)),
        });
    }
    DispatchResult::Continue
}
