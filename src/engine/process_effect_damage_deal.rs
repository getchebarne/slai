use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::types::DeltaSign;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;

pub fn process_effect_damage_deal(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
) {
    let id_target = id_target.expect("DamageDeal requires id_target");
    let from_card = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Card,
        None => false,
    };
    let mods_char = state.entities[state.id_character].modifiers;
    let id_character = state.id_character;

    let target = &mut state.entities[id_target];
    let damage_over_block = amount.saturating_sub(target.vitals.block);
    target.vitals.block = target.vitals.block.saturating_sub(amount);

    if damage_over_block > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: HealthDeltaAmount::Absolute(damage_over_block),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });

        // Envenom: card-played unblocked damage applies Poison; modifier-damage excluded
        if from_card && modifier_has(&mods_char, ModifierKind::Envenom) {
            let stacks = modifier_stacks(&mods_char, ModifierKind::Envenom);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Poison,
                    stacks,
                },
                id_source: Some(id_character),
                target: Target::Direct(Some(id_target)),
            });
        }

        if id_source != Some(id_target) {
            let target = &mut state.entities[id_target];
            fire_on_damage_taken(target, id_target, &mut state.effect_queue);
        }
    }
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
