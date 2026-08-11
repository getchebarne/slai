use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::active_modifier_kinds;
use crate::modifier::modifier_is_buff;
use crate::modifier::modifier_remove;

pub fn process_effect_debuffs_clear(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("DebuffsClear requires id_target");
    let modifiers = &mut state.entities[id_target].modifiers;

    for mod_kind in active_modifier_kinds(modifiers.active) {
        if !modifier_is_buff(mod_kind)
            || matches!(mod_kind, ModifierKind::Strength | ModifierKind::Dexterity)
                && modifiers.stacks[mod_kind as usize] < 0
        {
            modifier_remove(modifiers, mod_kind);
        }
    }
}
