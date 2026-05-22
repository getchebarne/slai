use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_def;
use crate::modifier::modifier_has;

// Multiply target's stacks of `kind` by `factor`. No-op if target doesn't have the modifier
pub fn process_effect_modifier_multiply(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: ModifierKind,
    factor: u8,
) {
    let id_target = id_target.expect("ModifierMultiply requires id_target");
    let modifiers = &mut state.entities[id_target].modifiers;
    if !modifier_has(modifiers, kind) {
        return;
    }
    let mod_def = modifier_def(kind);
    let stacks_cur = modifiers.stacks[kind as usize] as i32;
    let stacks_new =
        (stacks_cur * factor as i32).clamp(mod_def.stacks_min as i32, mod_def.stacks_max as i32);
    modifiers.stacks[kind as usize] = stacks_new as i16;
}
