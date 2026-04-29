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
    // Character death: abandon anything pending and mark dead so
    // derive_resting_phase returns Phase::GameOver on the natural drain.
    if id_target == id_character {
        entities[id_character].dead = true;
        queue.clear();
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
    // other alive enemy. `id_source = None` so no source-side scaling and
    // Envenom can't proc; block still subtracts
    let effects_corpse: Vec<Effect> =
        if modifier_has(&monster.modifiers, ModifierKind::CorpseExplosion) {
            let dmg = monster.vitals.health_max;
            id_monsters[..monster_count as usize]
                .iter()
                .filter(|&&id| id != id_target && !entities[id].dead)
                .map(|&id| Effect {
                    kind: EffectKind::DamageDeal { amount: dmg },
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
        for e in &effects_corpse {
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
        for e in effects_corpse.iter().rev() {
            queue.push_front(*e);
        }
    }

    DispatchResult::Continue
}
