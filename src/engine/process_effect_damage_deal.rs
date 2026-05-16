use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::types::Phase;

pub fn process_effect_damage_deal(
    entities: &mut [Entity],
    id_source: Option<usize>,
    id_character: usize,
    id_target: usize,
    amount: u16,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let from_card = match id_source {
        Some(id) => entities[id].kind == EntityKind::Card,
        None => false,
    };
    // Snapshot character modifiers (Modifiers is Copy) so the read borrow
    // doesn't alias the mut borrow on target taken below
    let mods_char = entities[id_character].modifiers;
    let target = &mut entities[id_target];

    let damage_over_block = amount.saturating_sub(target.vitals.block);
    target.vitals.block = target.vitals.block.saturating_sub(amount);

    if damage_over_block > 0 {
        effect_queue.push_front(Effect {
            kind: EffectKind::HealthLoss {
                amount: damage_over_block,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });

        // Envenom: when a card-played attack lands unblocked damage,
        // apply Envenom stacks of Poison to the target. `from_card` gates
        // out modifier-driven damage (e.g. ThousandCuts)
        if from_card && modifier_has(&mods_char, ModifierKind::Envenom) {
            let stacks = modifier_stacks(&mods_char, ModifierKind::Envenom);
            effect_queue.push_front(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Poison,
                    stacks,
                },
                id_source: Some(id_character),
                target: Target::Direct(Some(id_target)),
            });
        }

        // Target-side hook — fires only when actual HP loss > 0
        if id_source != Some(id_target) {
            fire_on_damage_taken(target, id_target, effect_queue);
        }
    }
    None
}

fn fire_on_damage_taken(
    target: &mut Entity,
    id_target: usize,
    effect_queue: &mut VecDeque<Effect>,
) {
    // CurlUp: gain block = stacks once per combat, then remove the modifier
    if modifier_has(&target.modifiers, ModifierKind::CurlUp) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::CurlUp);
        modifier_remove(&mut target.modifiers, ModifierKind::CurlUp);
        effect_queue.push_front(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_target),
            target: Target::Direct(Some(id_target)),
        });
    }

    // Angry: gain Strength = stacks every time it takes damage
    if modifier_has(&target.modifiers, ModifierKind::Angry) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::Angry);
        effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
}
