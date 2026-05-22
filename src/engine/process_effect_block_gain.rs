use crate::consts::FACTOR_FRAIL;
use crate::consts::MAX_BLOCK;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;

pub fn process_effect_block_gain(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
) {
    let id_target = id_target.expect("BlockGain requires id_target");
    // Card-played block scales with Dex/Frail; modifier-driven block (Metallicize,
    // PlatedArmor, AfterImage) and monster-self block do not
    let from_card = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Card,
        None => false,
    };

    let modifiers = &state.entities[id_target].modifiers;
    let mut value = amount as f32;

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
        let vitals = &mut state.entities[id_target].vitals;
        vitals.block = (vitals.block + final_block).min(MAX_BLOCK);
    }
}
