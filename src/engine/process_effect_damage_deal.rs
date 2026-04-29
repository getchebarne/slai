use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_remove, modifier_stacks};
use crate::types::Vitals;

pub fn process_effect_damage_deal(
    vitals: &mut Vitals,
    mods_target: &mut Modifiers,
    id_source: Option<usize>,
    id_target: usize,
    id_character: usize,
    mods_char: &Modifiers,
    amount: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let damage_over_block = amount.saturating_sub(vitals.block);
    vitals.block = vitals.block.saturating_sub(amount);

    if damage_over_block > 0 {
        queue.push_front(Effect {
            kind: EffectKind::HealthLoss {
                amount: damage_over_block,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });

        // Envenom (source-side): when player attack lands unblocked damage on a
        // non-self target, apply Envenom stacks of Poison to the target.
        // Stays inline — distinct from fire_on_damage_taken below which is
        // target-side (reading the target's own modifiers to react).
        if id_source == Some(id_character)
            && id_target != id_character
            && modifier_has(mods_char, ModifierKind::Envenom)
        {
            let stacks = modifier_stacks(mods_char, ModifierKind::Envenom);
            queue.push_front(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Poison,
                    stacks,
                },
                id_source: Some(id_character),
                target: Target::Direct(Some(id_target)),
            });
        }

        // Target-side hook — fires only when actual HP loss > 0 and only when
        // the source is a different entity (no self-damage). ModeShift's
        // damage-accumulator logic lives in process_effect_health_loss for
        // historical reasons; CurlUp and Angry slot in here.
        if id_source != Some(id_target) {
            fire_on_damage_taken(mods_target, id_target, queue);
        }
    }
    DispatchResult::Continue
}

fn fire_on_damage_taken(
    mods_target: &mut Modifiers,
    id_target: usize,
    queue: &mut VecDeque<Effect>,
) {
    // CurlUp: gain block = stacks once per combat, then remove the modifier.
    if modifier_has(mods_target, ModifierKind::CurlUp) {
        let stacks = modifier_stacks(mods_target, ModifierKind::CurlUp);
        modifier_remove(mods_target, ModifierKind::CurlUp);
        queue.push_front(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_target),
            target: Target::Direct(Some(id_target)),
        });
    }

    // Angry: gain Strength = stacks every time we take damage.
    if modifier_has(mods_target, ModifierKind::Angry) {
        let stacks = modifier_stacks(mods_target, ModifierKind::Angry);
        queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
}
