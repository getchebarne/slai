use crate::consts::FACTOR_FRAIL;
use crate::consts::MAX_BLOCK;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::types::Vitals;

pub fn process_effect_block_gain(
    vitals: &mut Vitals,
    modifiers: &Modifiers,
    amount: u16,
    from_card: bool,
) {
    let mut value = amount as f32;

    // Card-sourced block runs Dexterity then Frail
    if from_card {
        if modifier_has(modifiers, ModifierKind::Dexterity) {
            value += modifier_stacks(modifiers, ModifierKind::Dexterity) as f32;
        }
        if modifier_has(modifiers, ModifierKind::Frail) {
            value *= FACTOR_FRAIL;
        }
    }

    let final_block = value.max(0.0) as u16;
    if final_block > 0 {
        vitals.block = (vitals.block + final_block).min(MAX_BLOCK);
    }
}
