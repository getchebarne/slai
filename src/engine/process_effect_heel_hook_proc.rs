use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_has;
use crate::types::Phase;

// HeelHook bonus: if the targeted enemy has Weak, gain 1 energy and draw 1
// card. Modifier-check happens at handler time; the target may have died
// from the preceding DamagePhysical hit, but Weak still reads on dead
// entities so the bonus fires
//
// Push order: CardDraw first, then EnergyGain, so EnergyGain ends up at the
// front of the effect_queue and runs before CardDraw
pub fn process_effect_heel_hook_proc(
    mods_target: &Modifiers,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    if !modifier_has(mods_target, ModifierKind::Weak) {
        return None;
    }
    effect_queue.push_front(Effect {
        kind: EffectKind::CardDraw { count: 1 },
        id_source: None,
        target: Target::Direct(None),
    });
    effect_queue.push_front(Effect {
        kind: EffectKind::EnergyGain { amount: 1 },
        id_source: None,
        target: Target::Direct(None),
    });
    None
}
