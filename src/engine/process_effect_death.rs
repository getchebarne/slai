use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::monsters::Monster;
use crate::types::EntityId;

pub fn process_effect_death(
    actor: EntityId,
    monsters: &mut Vec<Monster>,
    character_id: EntityId,
) -> ProcessEffectResult {
    if actor == character_id {
        return ProcessEffectResult::Continue {
            top: vec![Effect::GameEnd],
            bot: Vec::new(),
        };
    }

    let idx = monsters.iter().position(|m| m.id == actor)
        .expect("Monster not found for Death");

    let mut effects = Vec::new();

    if modifier_has(&monsters[idx].vitals.modifiers, ModifierKind::SporeCloud) {
        let stacks =
            modifier_stacks(&monsters[idx].vitals.modifiers, ModifierKind::SporeCloud);
        effects.push(Effect::ModifierGain {
            target: character_id,
            kind: ModifierKind::Vulnerable,
            stacks,
        });
    }

    monsters.remove(idx);

    if monsters.is_empty() {
        effects.push(Effect::CombatEnd);
    }

    if effects.is_empty() {
        ProcessEffectResult::Pass
    } else {
        ProcessEffectResult::Continue {
            top: effects,
            bot: Vec::new(),
        }
    }
}
