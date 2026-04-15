use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::entities::Entity;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};

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

    let monster = &mut entities[id_target];

    // Modifier / SporeCloud (on-death effect)
    if modifier_has(&monster.modifiers, ModifierKind::SporeCloud) {
        let stacks = modifier_stacks(&monster.modifiers, ModifierKind::SporeCloud);
        effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks,
            },
            source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    monster.dead = true;

    let any_alive = id_monsters[..monster_count as usize]
        .iter()
        .any(|&id| !entities[id].dead);

    if !any_alive {
        effects.push(Effect {
            kind: EffectKind::CombatEnd,
            source: None,
            target: Target::Direct(None),
        });
        ProcessEffectResult::Replace(effects)
    } else if effects.is_empty() {
        ProcessEffectResult::Continue { top: vec![], bot: vec![] }
    } else {
        ProcessEffectResult::Continue {
            top: effects,
            bot: Vec::new(),
        }
    }
}
