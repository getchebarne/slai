use crate::consts::MAX_BLOCK;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::types::Vitals;

// TODO: frail
pub fn process_effect_block_gain(
    vitals: &mut Vitals,
    modifiers: &Modifiers,
    amount: u16,
    from_card: bool,
) -> ProcessEffectResult {
    let mut value = amount as i32;

    // Apply dextierity if the source is a card
    if from_card && modifier_has(modifiers, ModifierKind::Dexterity) {
        value += modifier_stacks(modifiers, ModifierKind::Dexterity) as i32;
    }

    // Sum block
    if value > 0 {
        vitals.block = (vitals.block + value as u16).min(MAX_BLOCK);
    }

    // Continue
    ProcessEffectResult::Continue { top: vec![], bot: vec![] }
}
