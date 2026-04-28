use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::types::Vitals;

pub fn process_effect_damage_deal(
    vitals: &mut Vitals,
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

        // Envenom: when player attack lands unblocked damage on a non-self
        // target, apply Envenom stacks of Poison to the target.
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
    }
    DispatchResult::Continue
}
