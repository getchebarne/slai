use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::state::Vitals;

// TODO: frail
pub fn process_effect_block_gain(
    vitals: &mut Vitals,
    amount: u16,
    from_card: bool,
) -> ProcessEffectResult {
    let mut value = amount as i32;

    if from_card && modifier_has(&vitals.modifiers, ModifierKind::Dexterity) {
        value += modifier_stacks(&vitals.modifiers, ModifierKind::Dexterity) as i32;
    }

    if value > 0 {
        vitals.block = (vitals.block + value as u16).min(999);
    }
    ProcessEffectResult::Pass
}
