use std::collections::VecDeque;

use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Initial Hexaghost hit
pub fn process_effect_hexaghost_divider(
    id_source: Option<usize>,
    id_character: usize,
    entities: &[Entity],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let health = entities[id_character].vitals.health;
    let dmg: u16 = health / 12 + 1;

    for _ in 0..HEXAGHOST_DIVIDER_HITS {
        effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: dmg },
            id_source,
            target: Target::Direct(Some(id_character)),
        });
    }
    DispatchResult::Continue
}
