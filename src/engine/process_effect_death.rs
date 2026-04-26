use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};

pub fn process_effect_death(
    id_target: usize,
    id_character: usize,
    id_monsters: &[usize],
    monster_count: u8,
    entities: &mut [Entity],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Character death: abandon anything pending, run only GameOver.
    if id_target == id_character {
        queue.clear();
        queue.push_back(Effect::direct(EffectKind::GameOver, None, None));
        return DispatchResult::Continue;
    }

    let monster = &entities[id_target];

    // SporeCloud: dying enemy stacks Vulnerable on the character.
    let spore_effect = if modifier_has(&monster.modifiers, ModifierKind::SporeCloud) {
        let stacks = modifier_stacks(&monster.modifiers, ModifierKind::SporeCloud);
        Some(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        })
    } else {
        None
    };

    // CorpseExplosion: dying enemy deals damage equal to its max HP to every
    // OTHER alive enemy. Uses DamagePower so source-side scaling does not
    // apply; target Vulnerable still multiplies; block still subtracts.
    let corpse_effects: Vec<Effect> = if modifier_has(&monster.modifiers, ModifierKind::CorpseExplosion) {
        let dmg = monster.vitals.health_max;
        id_monsters[..monster_count as usize]
            .iter()
            .filter(|&&id| id != id_target && !entities[id].dead)
            .map(|&id| Effect {
                kind: EffectKind::DamagePower { amount: dmg },
                id_source: None,
                target: Target::Direct(Some(id)),
            })
            .collect()
    } else {
        Vec::new()
    };

    entities[id_target].dead = true;

    let any_alive = id_monsters[..monster_count as usize]
        .iter()
        .any(|&id| !entities[id].dead);

    if !any_alive {
        // Combat ends. Replace pending effects with on-death triggers then CombatEnd.
        queue.clear();
        for e in &corpse_effects {
            queue.push_back(*e);
        }
        if let Some(e) = spore_effect {
            queue.push_back(e);
        }
        queue.push_back(Effect {
            kind: EffectKind::CombatEnd,
            id_source: None,
            target: Target::Direct(None),
        });
    } else {
        // Mid-combat: push to front so on-death triggers fire before any
        // suspended chain resumes.
        if let Some(e) = spore_effect {
            queue.push_front(e);
        }
        for e in corpse_effects.iter().rev() {
            queue.push_front(*e);
        }
    }

    DispatchResult::Continue
}
