use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::types::EntityId;
use crate::state::{Entity, EntityKind};

pub fn process_effect_death(
    actor: EntityId,
    character: EntityId,
    entities: &mut [Entity],
    monsters: &[EntityId],
    monster_count: u8,
) -> ProcessEffectResult {
    if actor == character {
        return ProcessEffectResult::Replace(vec![
            Effect::direct(EffectKind::GameOver, None, None),
        ]);
    }

    let mut effects = Vec::new();

    // Modifier / SporeCloud (on-death effect)
    {
        let EntityKind::Monster(m) = &entities[actor.0 as usize].kind else { unreachable!() };
        let modifiers = &m.modifiers;
        if modifier_has(modifiers, ModifierKind::SporeCloud) {
            let stacks = modifier_stacks(modifiers, ModifierKind::SporeCloud);
            effects.push(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Vulnerable,
                    stacks,
                },
                source: None,
                target: Target::Direct(Some(character)),
            });
        }
    }

    // Mark monster as dead
    let EntityKind::Monster(m) = &mut entities[actor.0 as usize].kind else { unreachable!() };
    m.dead = true;

    // If all monsters dead, replace queue w/ combat end
    let any_alive = monsters[..monster_count as usize]
        .iter()
        .any(|&id| {
            let EntityKind::Monster(m) = &entities[id.0 as usize].kind else { unreachable!() };
            !m.dead
        });

    if !any_alive {
        effects.push(Effect {
            kind: EffectKind::CombatEnd,
            source: None,
            target: Target::Direct(None),
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
