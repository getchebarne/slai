use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::utils::flush_effects_from_buf_to_queue_front;

// Reads move_current at dispatch (late binding); mid-turn overrides (split, wake) take effect
pub fn process_effect_move_execute(id_target: Option<usize>, state: &mut GameState) {
    let id_monster = id_target.expect("MoveExecute requires id_target");
    let entity = &state.entities[id_monster];
    // Corpses don't act: a mid-phase death leaves this queued effect dangling
    if entity.dead {
        return;
    }
    let Some(move_idx) = entity.monster_move_current else {
        return;
    };

    let stacks_thievery = if has_modifier(&entity.modifiers, ModifierKind::Thievery) {
        Some(modifier_stacks(&entity.modifiers, ModifierKind::Thievery) as u8)
    } else {
        None
    };

    let id_character = state.id_character;
    // Copy the Move out so effect_buf/queue mutations below don't hold `entities` borrowed
    let move_current = state.entities[id_monster].monster_moves[move_idx];
    state.effect_buf.clear();
    for effect in move_current.effects[..move_current.effects_len as usize].iter() {
        state.effect_buf.push(Effect {
            id_source: Some(id_monster),
            ..*effect
        });
        if let Some(amount) = stacks_thievery
            && matches!(effect.kind, EffectKind::DamagePhysical { .. })
        {
            state.effect_buf.push(Effect {
                kind: EffectKind::GoldSteal { amount },
                id_source: Some(id_monster),
                target: Target::Direct(Some(id_character)),
            });
        }
    }
    flush_effects_from_buf_to_queue_front(state);
}
