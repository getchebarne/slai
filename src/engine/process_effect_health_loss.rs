use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_def;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::monsters::lagavulin;
use crate::monsters::slime_acid_large;
use crate::monsters::slime_boss;
use crate::monsters::slime_spike_large;
use crate::types::MonsterName;

pub fn process_effect_health_loss(
    entity: &mut Entity,
    id_target: usize,
    id_character: usize,
    amount: u16,
    effect_queue: &mut VecDeque<Effect>,
) {
    // TODO: should only decrement for physical attacks
    if amount > 0 && modifier_has(&entity.modifiers, ModifierKind::PlatedArmor) {
        effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::PlatedArmor,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    entity.vitals.health = entity.vitals.health.saturating_sub(amount);

    if entity.vitals.health == 0 {
        effect_queue.push_front(Effect {
            kind: EffectKind::Death,
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        return;
    }

    // Splittable: post-HP-loss check fires on *any* damage source (attack or
    // poison). Override move_current so the next MoveExecute runs Split
    // instead of the slime's previously-selected attack. Consume the marker
    // so a multi-hit doesn't retrigger
    if modifier_has(&entity.modifiers, ModifierKind::Splittable)
        && entity.vitals.health <= entity.vitals.health_max / 2
    {
        let idx_split = match entity.monster_name {
            MonsterName::SlimeAcidLarge => slime_acid_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeSpikeLarge => slime_spike_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeBoss => slime_boss::IDX_MOVE_SPLIT,
            _ => panic!(
                "Splittable on unexpected monster: {:?}",
                entity.monster_name
            ),
        };
        entity.move_current = Some(idx_split);
        modifier_remove(&mut entity.modifiers, ModifierKind::Splittable);
    }

    // Asleep wake-via-HP-loss (Lagavulin): any damage including poison wakes
    // him. Set move_current = Stunned (one no-damage turn) and drop Asleep +
    // Metallicize
    if modifier_has(&entity.modifiers, ModifierKind::Asleep) {
        let stunned_idx = match entity.monster_name {
            MonsterName::Lagavulin => lagavulin::IDX_MOVE_STUNNED,
            _ => panic!(
                "Unsupported monster name for Asleep modifier: {:?}",
                entity.monster_name
            ),
        };
        entity.move_current = Some(stunned_idx);
        modifier_remove(&mut entity.modifiers, ModifierKind::Asleep);
        modifier_remove(&mut entity.modifiers, ModifierKind::Metallicize);
    }

    if modifier_has(&entity.modifiers, ModifierKind::ModeShift) {
        // ModeShift: damage reduces stacks, triggers move update on break
        let new_stacks =
            modifier_stacks(&entity.modifiers, ModifierKind::ModeShift) - amount as i16;
        if new_stacks < modifier_def(ModifierKind::ModeShift).stacks_min {
            modifier_remove(&mut entity.modifiers, ModifierKind::ModeShift);
            if id_target != id_character {
                effect_queue.push_front(Effect {
                    kind: EffectKind::MoveUpdate,
                    id_source: None,
                    target: Target::Direct(Some(id_target)),
                });
            }
        } else {
            entity.modifiers.stacks[ModifierKind::ModeShift as usize] = new_stacks;
        }
    }
}
