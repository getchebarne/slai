use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_apply;
use crate::modifier::modifier_def;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;

pub fn process_effect_modifier_gain(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: ModifierKind,
    stacks: i16,
) {
    let id_target = id_target.expect("ModifierGain requires id_target");
    let entity = &mut state.entities[id_target];
    let monster_cycle_count = match entity.kind {
        EntityKind::Monster => Some(entity.monster_cycle_count),
        _ => None,
    };
    let modifiers = &mut entity.modifiers;

    // ModeShift has special scaling logic
    if kind == ModifierKind::ModeShift {
        if let Some(cc) = monster_cycle_count {
            return process_mode_shift_gain(modifiers, stacks, cc);
        }
    }

    // Check artifact
    let is_debuff_attempt = (stacks > 0 && !modifier_def(kind).is_buff)
        || (stacks < 0 && (kind == ModifierKind::Dexterity || kind == ModifierKind::Strength));
    if is_debuff_attempt && modifier_has(modifiers, ModifierKind::Artifact) {
        let stacks_new = modifier_stacks(modifiers, ModifierKind::Artifact) - 1;
        if stacks_new < modifier_def(ModifierKind::Artifact).stacks_min {
            modifier_remove(modifiers, ModifierKind::Artifact);
        } else {
            modifiers.stacks[ModifierKind::Artifact as usize] = stacks_new;
        }
        return;
    }

    // Negative stacks reduce existing modifier, removing if below minimum
    if stacks < 0 {
        if modifier_has(modifiers, kind) {
            let idx = kind as usize;
            modifiers.stacks[idx] += stacks;
            let mod_def = modifier_def(kind);
            if modifiers.stacks[idx] < mod_def.stacks_min {
                modifier_remove(modifiers, kind);
            }
        } else if modifier_def(kind).stacks_min < 0 {
            modifier_apply(modifiers, kind, stacks);
        }
        return;
    }

    modifier_apply(modifiers, kind, stacks);
}

fn process_mode_shift_gain(modifiers: &mut Modifiers, stacks: i16, monster_cycle_count: u8) {
    let increase = MODE_SHIFT_INCREASE_PER_CYCLE * monster_cycle_count as i16;
    modifier_apply(modifiers, ModifierKind::ModeShift, stacks + increase);
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;
}
