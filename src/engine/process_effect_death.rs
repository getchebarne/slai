use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::entities::{Entity, EntityKind};

pub fn process_effect_death(
    id_target: usize,
    id_character: usize,
    id_monsters: &[usize],
    monster_count: u8,
    entities: &mut [Entity],
) -> ProcessEffectResult {
    if id_target == id_character {
        return ProcessEffectResult::Replace(vec![Effect::direct(
            EffectKind::GameOver,
            None,
            None,
        )]);
    }

    let mut effects = Vec::new();

    let EntityKind::Monster(m) = &mut entities[id_target].kind else {
        unreachable!()
    };

    // Modifier / SporeCloud (on-death effect)
    if modifier_has(&m.modifiers, ModifierKind::SporeCloud) {
        let stacks = modifier_stacks(&m.modifiers, ModifierKind::SporeCloud);
        effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks,
            },
            source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    m.dead = true;

    // If all monsters dead, replace queue w/ combat end
    let any_alive = id_monsters[..monster_count as usize].iter().any(|&id| {
        let EntityKind::Monster(m) = &entities[id].kind else {
            unreachable!()
        };
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
