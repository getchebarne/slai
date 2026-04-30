use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_remove, modifier_stacks};
use crate::monsters::{lagavulin, slime_acid_large, slime_boss, slime_spike_large};
use crate::types::MonsterName;

pub fn process_effect_damage_deal(
    target: &mut Entity,
    id_source: Option<usize>,
    id_target: usize,
    id_character: usize,
    mods_char: &Modifiers,
    amount: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let damage_over_block = amount.saturating_sub(target.vitals.block);
    target.vitals.block = target.vitals.block.saturating_sub(amount);

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
        // historical reasons; CurlUp, Angry, and Splittable slot in here.
        if id_source != Some(id_target) {
            fire_on_damage_taken(target, id_target, queue);
        }
    }
    DispatchResult::Continue
}

fn fire_on_damage_taken(target: &mut Entity, id_target: usize, queue: &mut VecDeque<Effect>) {
    // CurlUp: gain block = stacks once per combat, then remove the modifier.
    if modifier_has(&target.modifiers, ModifierKind::CurlUp) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::CurlUp);
        modifier_remove(&mut target.modifiers, ModifierKind::CurlUp);
        queue.push_front(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_target),
            target: Target::Direct(Some(id_target)),
        });
    }

    // Angry: gain Strength = stacks every time we take damage.
    if modifier_has(&target.modifiers, ModifierKind::Angry) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::Angry);
        queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Splittable: when health drops to ≤ 50% (and the L is still alive),
    // override move_current to the per-monster Split move and consume the
    // Splittable marker so a multi-hit doesn't retrigger.
    if modifier_has(&target.modifiers, ModifierKind::Splittable)
        && target.vitals.health > 0
        && target.vitals.health <= target.vitals.health_max / 2
    {
        let split_idx = match target.monster_name {
            MonsterName::SlimeAcidLarge => slime_acid_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeSpikeLarge => slime_spike_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeBoss => slime_boss::IDX_MOVE_SPLIT,
            _ => return,
        };
        target.move_current = Some(split_idx);
        modifier_remove(&mut target.modifiers, ModifierKind::Splittable);
    }

    // Asleep wake-via-damage (Lagavulin): on HP loss while Asleep, set
    // move_current = Stunned (one no-damage monster turn) and remove Asleep +
    // Metallicize. Subsequent damage instances see no Asleep → fall through.
    if modifier_has(&target.modifiers, ModifierKind::Asleep) && target.vitals.health > 0 {
        let stunned_idx = match target.monster_name {
            MonsterName::Lagavulin => lagavulin::IDX_MOVE_STUNNED,
            _ => return,
        };
        target.move_current = Some(stunned_idx);
        modifier_remove(&mut target.modifiers, ModifierKind::Asleep);
        modifier_remove(&mut target.modifiers, ModifierKind::Metallicize);
    }
}
