use crate::effect::{Effect, EffectKind};
use crate::engine::{HaltReason, ProcessEffectResult};
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::state::Entity;
use crate::types::EntityId;

pub fn process_effect_death(
    actor: EntityId,
    character: EntityId,
    entities: &mut [Entity],
    monsters: &[EntityId],
    monster_count: u8,
) -> ProcessEffectResult {
    // Character death ends the game
    if actor == character {
        return ProcessEffectResult::Halt(HaltReason::GameOver);
    }

    let mut effects = Vec::new();

    // Modifier / SporeCloud (on-death effect)
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
                target: Some(character),
            });
        }
    }

    // Mark monster as dead
    entities[actor.0 as usize].kind.monster_mut().dead = true;

    // If all monsters dead, replace queue w/ combat end
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
