use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::types::Phase;

// Subtract 1 because card_play increments the counter before this effect fires
pub fn process_effect_finisher_damage(
    this_turn_attacks_played: u8,
    id_source: Option<usize>,
    id_target: usize,
    damage: u16,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let num_attacks = this_turn_attacks_played.saturating_sub(1);
    for _ in 0..num_attacks {
        effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: damage },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
    None
}
