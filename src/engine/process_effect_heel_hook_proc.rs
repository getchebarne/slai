use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has};

// HeelHook bonus: if the targeted enemy has Weak, gain 1 energy and draw 1
// card. Modifier-check happens at handler time; the target may have died
// from the preceding DamagePhysical hit, but Weak still reads on dead
// entities so the bonus fires (matches StS HeelHookAction).
//
// Push order: CardDraw first, then EnergyGain, so EnergyGain ends up at the
// front of the queue and runs before CardDraw (matches StS addToTop order).
pub fn process_effect_heel_hook_proc(
    target_mods: &Modifiers,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if !modifier_has(target_mods, ModifierKind::Weak) {
        return DispatchResult::Continue;
    }
    queue.push_front(Effect {
        kind: EffectKind::CardDraw { count: 1 },
        id_source: None,
        target: Target::Direct(None),
    });
    queue.push_front(Effect {
        kind: EffectKind::EnergyGain { amount: 1 },
        id_source: None,
        target: Target::Direct(None),
    });
    DispatchResult::Continue
}
