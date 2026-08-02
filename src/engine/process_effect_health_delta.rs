use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_def;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::monsters::lagavulin;
use crate::monsters::slime_acid_large;
use crate::monsters::slime_boss;
use crate::monsters::slime_spike_large;
use crate::monsters::the_guardian;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_health_delta(
    id_target: Option<usize>,
    state: &mut GameState,
    sign: DeltaSign,
    amount: Amount,
) {
    let id_target = id_target.expect("HealthDelta requires id_target");

    // Resolve amount
    let amount = match amount {
        Amount::Absolute(a) => a,
        Amount::Relative {
            numerator,
            denominator,
        }
        | Amount::RelativeRounded {
            numerator,
            denominator,
        }
        | Amount::RelativeCeil {
            numerator,
            denominator,
        } => {
            let health_max = state.entities[id_target].vitals.health_max;
            // f32 mirrors the source's (int)(maxHP * fraction) float truncation
            let mut raw = health_max as f32 * (numerator as f32 / denominator as f32);
            match amount {
                Amount::RelativeRounded { .. } => raw += 0.5,
                Amount::RelativeCeil { .. } => raw = raw.ceil(),
                _ => {}
            }
            let raw = raw as u32;
            match sign {
                DeltaSign::Loss => raw.max(1) as u16,
                DeltaSign::Gain => raw as u16,
            }
        }
        _ => {
            unreachable!("HealthDelta resolves only Absolute or Relative amounts")
        }
    };

    // Apply amount
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
    // Buffer: absorb one HP-loss instance outright, before anything reacts to it
    if amount > 0 {
        let modifiers = &mut state.entities[id_target].modifiers;
        if has_modifier(modifiers, ModifierKind::Buffer) {
            if modifier_stacks(modifiers, ModifierKind::Buffer) <= 1 {
                modifier_remove(modifiers, ModifierKind::Buffer);
            } else {
                modifiers.stacks[ModifierKind::Buffer as usize] -= 1;
            }
            return;
        }
    }

    // Tungsten Rod: every HP loss is reduced by 1, before anything reacts to it
    let amount = if id_target == state.id_character
        && amount > 0
        && has_relic(&state.id_relics, RelicName::TungstenRod)
    {
        amount - 1
    } else {
        amount
    };

    // Centennial Puzzle: the first actual HP loss each combat draws 3
    if id_target == state.id_character
        && amount > 0
        && matches!(state.mode_stack.last(), Some(Mode::Combat { .. }))
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

    // Bump number of damage instances taken this combat
    if id_target == state.id_character
        && amount > 0
        && let Some(Mode::Combat {
            this_combat_damage_instances_taken,
            ..
        }) = state.mode_stack.last_mut()
    {
        *this_combat_damage_instances_taken = this_combat_damage_instances_taken.saturating_add(1);
    }

    // Get mutable target reference
    let target = &mut state.entities[id_target];

    // Plated Armor: Decrement stacks
    // TODO: should only decrement for physical attacks
    if amount > 0 && has_modifier(&target.modifiers, ModifierKind::PlatedArmor) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::PlatedArmor,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Decrement health
    target.vitals.health = target.vitals.health.saturating_sub(amount);

    // Check if the target's dead. If so, queue death effect and return early
    if target.vitals.health == 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::Death,
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        return;
    }

    // Splittable: any damage at <= health_max / 2 overrides next move to Split
    if has_modifier(&target.modifiers, ModifierKind::Splittable)
        && target.vitals.health <= target.vitals.health_max / 2
    {
        let idx_split = match target.monster_name {
            MonsterName::SlimeAcidLarge => slime_acid_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeSpikeLarge => slime_spike_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeBoss => slime_boss::IDX_MOVE_SPLIT,
            _ => panic!(
                "Splittable on unexpected monster: {:?}",
                target.monster_name
            ),
        };
        // Executes in reverse:
        //     1. ModifierRemove Splittable
        //     2. MoveUpdate (Split)
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: Some(idx_split),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Splittable,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Lagavulin: any HP loss wakes him up
    if has_modifier(&target.modifiers, ModifierKind::Asleep) {
        let idx_stunned = match target.monster_name {
            MonsterName::Lagavulin => lagavulin::IDX_MOVE_STUNNED,
            _ => panic!(
                "Unsupported monster name for Asleep modifier: {:?}",
                target.monster_name
            ),
        };
        // Executes in reverse:
        //     1. ModifierRemove Metallicize
        //     2. ModifierRemove Asleep
        //     3. MoveUpdate (stunned)
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: Some(idx_stunned),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Asleep,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Metallicize,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Mode Shift (The Guardian): Damage reduces stacks, triggers move update on break
    if has_modifier(&target.modifiers, ModifierKind::ModeShift) {
        let new_stacks =
            modifier_stacks(&target.modifiers, ModifierKind::ModeShift) - amount as i16;

        if new_stacks < modifier_def(ModifierKind::ModeShift).stacks_min {
            modifier_remove(&mut target.modifiers, ModifierKind::ModeShift);
            if id_target != state.id_character {
                // Executes in reverse:
                //     1. BlockGain
                //     2. MoveUpdate
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::MoveUpdate {
                        move_override: None,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_target)),
                });

                // Entering Defensive Mode grants block before the move swap resolves
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::BlockGain {
                        amount: the_guardian::DEFENSIVE_MODE_BLOCK,
                    },
                    id_source: Some(id_target),
                    target: Target::Direct(Some(id_target)),
                });
            } else {
                panic!("Tried to remove Mode Shift from the Character")
            }
        } else {
            target.modifiers.stacks[ModifierKind::ModeShift as usize] = new_stacks;
        }
    }
}
