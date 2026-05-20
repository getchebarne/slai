use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::types::CardKind;

// Flechettes: deal `damage` per Skill currently in hand. Hand snapshot at
// handler time; Flechettes itself was already moved to discard by card_play
// so it can't be in hand. push_front in reverse so the resulting effect_queue runs
// the hits in any order (order doesn't matter — same target, same damage)
pub fn process_effect_flechettes_damage(
    entities: &[Entity],
    id_hand: &[usize],
    id_source: Option<usize>,
    id_target: usize,
    damage: u16,
    effect_queue: &mut VecDeque<Effect>,
) {
    let num_skills_in_hand = id_hand
        .iter()
        .filter(|&&id| entities[id].card_kind == CardKind::Skill)
        .count();
    for _ in 0..num_skills_in_hand {
        effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: damage },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
}
