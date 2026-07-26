use crate::consts::MAX_BLOCK;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::utils::scale_block_gain;

pub fn process_effect_block_gain(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
) {
    let id_target = id_target.expect("BlockGain requires id_target");
    // Card-played block scales with Dex/Frail; modifier-driven and monster-self don't
    let from_card = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Card,
        None => false,
    };

    // No-Block (Panic Button): card-sourced block on the character is negated
    if from_card
        && id_target == state.id_character
        && has_modifier(&state.entities[id_target].modifiers, ModifierKind::NoBlock)
    {
        return;
    }

    let modifiers = &state.entities[id_target].modifiers;
    let final_block = if from_card {
        let dex = if has_modifier(modifiers, ModifierKind::Dexterity) {
            modifier_stacks(modifiers, ModifierKind::Dexterity)
        } else {
            0
        };
        scale_block_gain(amount, dex, has_modifier(modifiers, ModifierKind::Frail))
    } else {
        amount
    };

    if final_block > 0 {
        let vitals = &mut state.entities[id_target].vitals;
        vitals.block = (vitals.block + final_block).min(MAX_BLOCK);
    }
}
