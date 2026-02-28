use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::state::{Entity, EntityKind};
use crate::types::EntityId;

pub fn process_effect_death(
    actor: EntityId,
    entities: &mut [Option<Entity>],
) -> ProcessEffectResult {
    if actor.0 == 0 {
        return ProcessEffectResult::Continue {
            top: vec![Effect::GameEnd],
            bot: Vec::new(),
        };
    }

    let mut effects = Vec::new();

    if let Some(entity) = &entities[actor.0 as usize] {
        let (_, modifiers) = entity.kind.combatant_ref();
        if modifier_has(modifiers, ModifierKind::SporeCloud) {
            let stacks = modifier_stacks(modifiers, ModifierKind::SporeCloud);
            effects.push(Effect::ModifierGain {
                target: EntityId(0),
                kind: ModifierKind::Vulnerable,
                stacks,
            });
        }
    }

    entities[actor.0 as usize] = None;

    let any_monsters_alive = entities.iter().any(|s|
        matches!(s, Some(Entity { kind: EntityKind::Monster(..) }))
    );
    if !any_monsters_alive {
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
