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
    let Some(move_idx) = entity.monster_move_current else {
        return;
    };

    let stacks_thievery = if has_modifier(&entity.modifiers, ModifierKind::Thievery) {
        Some(modifier_stacks(&entity.modifiers, ModifierKind::Thievery) as u8)
    } else {
        None
    };

    let id_character = state.id_character;
    let effects: &'static [Effect] = state.entities[id_monster].monster_moves[move_idx].effects;
    state.effect_buf.clear();
    for e in effects.iter() {
        state.effect_buf.push(Effect {
            id_source: Some(id_monster),
            ..*e
        });
        if let Some(amount) = stacks_thievery
            && matches!(e.kind, EffectKind::DamagePhysical { .. })
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
