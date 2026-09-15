use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;

// Multiply target's stacks of `kind` by `factor`. No-op if target doesn't have the modifier
pub fn process_effect_modifier_multiply(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: ModifierKind,
    factor: u8,
) {
    let id_target = id_target.expect("ModifierMultiply requires id_target");
    let modifiers = &state.entities[id_target].modifiers;
    if !has_modifier(modifiers, kind) {
        return;
    }

    // Calculate stacks' delta
    let stacks_cur = modifier_stacks(modifiers, kind) as i32;
    let delta = stacks_cur * (factor as i32 - 1);
    if delta == 0 {
        return;
    }

    // Enqueue `ModifierGain` so that Snecko Skull, Sadistic Nature, etc. also proc
    state.effect_queue.push_front(Effect {
        kind: EffectKind::ModifierGain {
            kind,
            stacks: delta as i16,
        },
        id_source: Some(state.id_character),
        target: Target::Direct(Some(id_target)),
    });
}
