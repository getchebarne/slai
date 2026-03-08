use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::state::Entity;
use crate::types::EntityId;

pub fn process_effect_death(
    actor: EntityId,
    entities: &mut [Entity],
    monsters: &[EntityId],
    monster_count: u8,
) -> ProcessEffectResult {
    if actor.0 == 0 {
        return ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::GameEnd,
                source: None,
                target: None,
            }],
            bot: Vec::new(),
        };
    }

    let mut effects = Vec::new();

    {
        let (_, modifiers) = entities[actor.0 as usize].kind.combatant_ref();
        if modifier_has(modifiers, ModifierKind::SporeCloud) {
            let stacks = modifier_stacks(modifiers, ModifierKind::SporeCloud);
            effects.push(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Vulnerable,
                    stacks,
                },
                source: None,
                target: Some(EntityId(0)),
            });
        }
    }

    entities[actor.0 as usize].kind.monster_mut().dead = true;

    let any_alive = monsters[..monster_count as usize]
        .iter()
        .any(|&id| !entities[id.0 as usize].kind.monster_ref().dead);

    if !any_alive {
        effects.push(Effect {
            kind: EffectKind::CombatEnd,
            source: None,
            target: None,
        });
        ProcessEffectResult::Replace(effects)
    } else if effects.is_empty() {
        ProcessEffectResult::Continue
    } else {
        ProcessEffectResult::AddAndContinue {
            top: effects,
            bot: Vec::new(),
        }
    }
}
