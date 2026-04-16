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

    let monster = &mut entities[id_target];

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

    monster.dead = true;

    let any_alive = id_monsters[..monster_count as usize]
        .iter()
        .any(|&id| !entities[id].dead);

    if !any_alive {
        // Combat ends. Replace pending effects with SporeCloud (if any) then CombatEnd.
        queue.clear();
        if let Some(e) = spore_effect {
            queue.push_back(e);
        }
        queue.push_back(Effect {
            kind: EffectKind::CombatEnd,
            id_source: None,
            target: Target::Direct(None),
        });
    } else if let Some(e) = spore_effect {
        queue.push_front(e);
    }

    DispatchResult::Continue
}
