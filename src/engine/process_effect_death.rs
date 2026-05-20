use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;

pub fn process_effect_death(
    id_target: usize,
    id_character: usize,
    id_monsters: &[usize],
    monster_count: u8,
    entities: &mut [Entity],
    effect_queue: &mut VecDeque<Effect>,
) {
    // Character death: clear pending work and mark dead (process_queue exits
    // on the dead flag; FFI derives GameOver from it)
    if id_target == id_character {
        entities[id_character].dead = true;
        effect_queue.clear();
        return;
    }

    // Monster-only path
    let monster = &entities[id_target];

    // Stolen-gold return
    let gold_return = if monster.monster_stolen_gold > 0 {
        Some(Effect {
            kind: EffectKind::GoldGain {
                amount: monster.monster_stolen_gold,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        })
    } else {
        None
    };

    // SporeCloud: dying enemy stacks Vulnerable on the character
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
        // Combat ends. Replace pending effects with on-death triggers then CombatEnd
        effect_queue.clear();
        if let Some(e) = gold_return {
            effect_queue.push_back(e);
        }
        for e in &effects_corpse {
            effect_queue.push_back(*e);
        }
        if let Some(e) = spore_effect {
            effect_queue.push_back(e);
        }
        effect_queue.push_back(Effect {
            kind: EffectKind::CombatEnd,
            id_source: None,
            target: Target::Direct(None),
        });
    } else {
        // Mid-combat: push to front so on-death triggers fire before any
        // suspended chain resumes
        if let Some(e) = spore_effect {
            effect_queue.push_front(e);
        }
        for e in effects_corpse.iter().rev() {
            effect_queue.push_front(*e);
        }
        if let Some(e) = gold_return {
            effect_queue.push_front(e);
        }
    }
}
