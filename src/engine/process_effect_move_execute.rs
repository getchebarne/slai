use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::EffectBuf;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::types::Phase;

// Dynamic move resolution: read `move_current` at dispatch time and push the
// move's effects onto the queue. Used so that mid-turn move overrides (slime
// split / Lagavulin wake on poison) actually take effect — at turn_end the
// monster's slot is queued as a single `MoveExecute`, not as the inline
// effects of whatever move was selected last turn
pub fn process_effect_move_execute(
    entity: &Entity,
    id_monster: usize,
    id_character: usize,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let Some(move_idx) = entity.move_current else {
        return None;
    };

    let stacks_thievery = if modifier_has(&entity.modifiers, ModifierKind::Thievery) {
        Some(modifier_stacks(&entity.modifiers, ModifierKind::Thievery) as u8)
    } else {
        None
    };

    let mut buf = EffectBuf::new();
    for e in entity.moves[move_idx].effects.iter() {
        buf.push(Effect {
            id_source: Some(id_monster),
            ..*e
        });
        if let Some(amount) = stacks_thievery
            && matches!(e.kind, EffectKind::DamagePhysical { .. })
        {
            buf.push(Effect {
                kind: EffectKind::GoldSteal { amount },
                id_source: Some(id_monster),
                target: Target::Direct(Some(id_character)),
            });
        }
    }
    buf.push_all_front(effect_queue);
    None
}
