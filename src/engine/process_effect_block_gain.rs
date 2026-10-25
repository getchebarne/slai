use crate::consts::MAX_BLOCK;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::utils::scale_block_gain;

pub fn process_effect_block_gain(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    amount: u16,
) {
    assert!(!id_targets.is_empty(), "BlockGain requires id_target");

    // Card-played block scales with Dex/Frail; modifier-driven and Monster-self don't
    let from_card = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Card,
        None => false,
    };
    for &id_target in id_targets {
        // Dead Entities don't block
        if state.entities[id_target].dead {
            continue;
        }

        // No-Block (Panic Button): Card-sourced block on the Character is negated
        if from_card
            && id_target == state.id_character
            && has_modifier(&state.entities[id_target].modifiers, ModifierKind::NoBlock)
        {
            continue;
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
}
