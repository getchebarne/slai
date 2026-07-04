use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
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
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::Screen;

pub fn process_effect_health_delta(
    id_target: Option<usize>,
    state: &mut GameState,
    sign: DeltaSign,
    amount: HealthDeltaAmount,
) {
    let id_target = id_target.expect("HealthDelta requires id_target");
    let amount = match amount {
        HealthDeltaAmount::Absolute(a) => a,
        HealthDeltaAmount::Relative {
            numerator,
            denominator,
        } => {
            let health_max = state.entities[id_target].vitals.health_max;
            let raw = (health_max as u32 * numerator as u32) / denominator as u32;
            match sign {
                DeltaSign::Loss => raw.max(1) as u16,
                DeltaSign::Gain => raw as u16,
            }
        }
    };
    match sign {
        DeltaSign::Gain => apply_gain(id_target, state, amount),
        DeltaSign::Loss => apply_loss(id_target, state, amount),
    }
}

fn apply_gain(id_target: usize, state: &mut GameState, amount: u16) {
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health = (vitals.health + amount).min(vitals.health_max);
}

fn apply_loss(id_target: usize, state: &mut GameState, amount: u16) {
    // Tungsten Rod: every HP loss is reduced by 1, before anything reacts to it
    let amount = if id_target == state.id_character
        && amount > 0
        && state.id_relics[RelicName::TungstenRod as usize].is_some()
    {
        amount - 1
    } else {
        amount
    };

    // Centennial Puzzle: the first actual HP loss each combat draws 3
    if id_target == state.id_character
        && amount > 0
        && matches!(state.screen, Screen::Combat)
        && let Some(id_relic) = state.id_relics[RelicName::CentennialPuzzle as usize]
        && state.entities[id_relic].relic_counter == 0
    {
        state.entities[id_relic].relic_counter = 1;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: 3 },
            id_source: None,
            target: Target::Direct(None),
        });
    }

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
        entity.monster_move_current = Some(idx_split);
        modifier_remove(&mut entity.modifiers, ModifierKind::Splittable);
    }

    // Lagavulin: any HP loss wakes him -> Stunned move, drop Asleep + Metallicize
    if modifier_has(&entity.modifiers, ModifierKind::Asleep) {
        let stunned_idx = match entity.monster_name {
            MonsterName::Lagavulin => lagavulin::IDX_MOVE_STUNNED,
            _ => panic!(
                "Unsupported monster name for Asleep modifier: {:?}",
                entity.monster_name
            ),
        };
        entity.monster_move_current = Some(stunned_idx);
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

#[cfg(test)]
mod tests {
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::HealthDeltaAmount;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::engine::test_support::combat_with_relic;
    use crate::game::GameState;
    use crate::types::DeltaSign;
    use crate::types::MonsterName;
    use crate::types::RelicName;

    fn lose_hp(state: &mut GameState, amount: u16) {
        let id_character = state.id_character;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: HealthDeltaAmount::Absolute(amount),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
        process_effect_queue(state);
    }

    #[test]
    fn tungsten_rod_shaves_one_off_every_loss() {
        let mut state = combat_with_relic(RelicName::TungstenRod, MonsterName::JawWorm);
        let id_character = state.id_character;
        let hp_before = state.entities[id_character].vitals.health;
        lose_hp(&mut state, 1);
        assert_eq!(state.entities[id_character].vitals.health, hp_before);
        lose_hp(&mut state, 5);
        assert_eq!(state.entities[id_character].vitals.health, hp_before - 4);
    }

    #[test]
    fn centennial_puzzle_draws_on_first_loss_only() {
        let mut state = combat_with_relic(RelicName::CentennialPuzzle, MonsterName::JawWorm);
        let hand_before = state.id_hand.len();
        lose_hp(&mut state, 3);
        assert_eq!(state.id_hand.len(), hand_before + 3);
        lose_hp(&mut state, 3);
        assert_eq!(state.id_hand.len(), hand_before + 3);
    }
}
