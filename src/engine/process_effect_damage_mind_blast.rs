use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::types::Phase;

// Damage equals draw-pile size at play time
pub fn process_effect_damage_mind_blast(
    id_source: Option<usize>,
    id_target: usize,
    pile_draw_size: usize,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    effect_queue.push_front(Effect {
        kind: EffectKind::DamagePhysical {
            amount: pile_draw_size as u16,
        },
        id_source,
        target: Target::Direct(Some(id_target)),
    });
    None
}
