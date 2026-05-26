use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::Target;
use crate::game::GameState;
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
use crate::types::Screen;

pub fn process_effect_health_delta(
    id_target: Option<usize>,
    state: &mut GameState,
    sign: HealthDeltaSign,
    amount: HealthDeltaAmount,
) {
    // Pct ignores id_target and resolves against the character
    let (id_target, amount) = match amount {
        HealthDeltaAmount::Flat(a) => (id_target.expect("HealthDelta Flat requires id_target"), a),
        HealthDeltaAmount::Pct { numer, denom } => {
            let id = state.id_character;
            let health_max = state.entities[id].vitals.health_max;
            let raw = (health_max as u32 * numer as u32) / denom as u32;
            let a = match sign {
                HealthDeltaSign::Loss => raw.max(1) as u16,
                HealthDeltaSign::Gain => raw as u16,
            };
            (id, a)
        }
    };
    match sign {
        HealthDeltaSign::Gain => apply_gain(id_target, state, amount),
        HealthDeltaSign::Loss => apply_loss(id_target, state, amount),
    }
}

fn apply_gain(id_target: usize, state: &mut GameState, amount: u16) {
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health = (vitals.health + amount).min(vitals.health_max);
}

fn apply_loss(id_target: usize, state: &mut GameState, amount: u16) {
    // MasterfulStab GrowsOnDamageInstanceTaken: bump per post-block character damage
    if id_target == state.id_character && amount > 0 && matches!(state.screen, Screen::Combat) {
        state.this_combat_damage_instances_taken =
            state.this_combat_damage_instances_taken.saturating_add(1);
    }

    let entity = &mut state.entities[id_target];

    // TODO: should only decrement for physical attacks
    if amount > 0 && modifier_has(&entity.modifiers, ModifierKind::PlatedArmor) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::PlatedArmor,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    let entity = &mut state.entities[id_target];
    entity.vitals.health = entity.vitals.health.saturating_sub(amount);

    if entity.vitals.health == 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::Death,
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        return;
    }

    // Splittable: any damage at ≤½ HP overrides next MoveExecute to Split; consume marker
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

    // Lagavulin: any HP loss wakes him → Stunned move, drop Asleep + Metallicize
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
            if id_target != state.id_character {
                state.effect_queue.push_front(Effect {
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
