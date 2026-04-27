use crate::consts::{FACTOR_FRAIL, MAX_BLOCK};
use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::types::Vitals;

pub fn process_effect_block_gain(
    vitals: &mut Vitals,
    modifiers: &Modifiers,
    amount: u16,
    from_card: bool,
) -> DispatchResult {
    let mut value = amount as f32;

    // Card-sourced block runs Dexterity then Frail (StS order: dex adds, frail multiplies).
    // Monster-sourced block (Lagavulin Metallicize, Shield Gremlin Protect) skips both.
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

    DispatchResult::Continue
}
