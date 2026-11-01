use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;

pub fn process_effect_modifier_strength_negative_remove(
    id_target: Option<usize>,
    state: &mut GameState,
) {
    let id_target = id_target.expect("ModifierStrengthNegativeRemove requires id_target");
    let mods = &mut state.entities[id_target].modifiers;
    if modifier_stacks(mods, ModifierKind::Strength) < 0 {
        modifier_remove(mods, ModifierKind::Strength);
    }
}
