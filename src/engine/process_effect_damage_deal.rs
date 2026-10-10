use std::collections::VecDeque;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_apply;
use crate::modifier::modifier_stacks;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_damage_deal(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
    lifesteal: bool, // Life Suck
) {
    let id_target = id_target.expect("DamageDeal requires id_target");

    // Corpses absorb nothing: pre-resolved Direct hits can outlive their target
    if state.entities[id_target].dead {
        return;
    }

    // Get source entity type
    let from_card = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Card,
        None => false,
    };
    let from_monster = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Monster,
        None => false,
    };

    // Intangible clamps the instance before block is spent against it
    let amount = if amount > 1
        && has_modifier(
            &state.entities[id_target].modifiers,
            ModifierKind::Intangible,
        ) {
        1
    } else {
        amount
    };

    // Substract block and calculate damage over it
    let target = &mut state.entities[id_target];
    let block_prev = target.vitals.block;
    let mut damage_over_block = amount.saturating_sub(block_prev);
    target.vitals.block = block_prev.saturating_sub(amount);

    // Boot: the player's Card hits leaving a 1-4 remainder land 5 instead
    if from_card
        && target.kind == EntityKind::Monster
        && 1 <= damage_over_block
        && damage_over_block <= 4
        && has_relic(&state.id_relics, RelicName::Boot)
    {
        damage_over_block = 5;
    }

    // Torii: Monster hits leaving a 2-5 remainder on the Character land 1 instead
    let id_character = state.id_character;
    if from_monster
        && id_target == id_character
        && 2 <= damage_over_block
        && damage_over_block <= 5
        && has_relic(&state.id_relics, RelicName::Torii)
    {
        damage_over_block = 1;
    }

    // Hand Drill: breaking a Monster's block applies 2 Vulnerable
    if target.kind == EntityKind::Monster
        && block_prev > 0
        && target.vitals.block == 0
        && has_relic(&state.id_relics, RelicName::HandDrill)
    {
        // The Character applies it behind everything queued, the rest of a Card included
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_target)),
        });
    }

    // Executes in reverse:
    //     1. On-damage-taken triggers (Angry; CurlUp / Flight / Malleable tail-queue)
    //     2. HealthDelta
    //     3. ModifierDelta Poison (Envenom)
    //     4. HealthDelta Gain (lifesteal)
    if damage_over_block > 0 {
        if lifesteal && let Some(id_source) = id_source {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::LifestealHeal,
                id_source: None,
                target: Target::Direct(Some(id_source)),
            });
        }

        // Envenom: Card-played unblocked damage applies Poison; modifier-damage excluded
        let mods_char = state.entities[state.id_character].modifiers;
        if from_card && has_modifier(&mods_char, ModifierKind::Envenom) {
            let stacks = modifier_stacks(&mods_char, ModifierKind::Envenom);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Poison,
                    stacks,
                },
                id_source: Some(id_character),
                target: Target::Direct(Some(id_target)),
            });
        }

        // The attacker's ID rides along: the HP loss reads it
        state.effect_queue.push_front(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(damage_over_block),
            },
            id_source,
            target: Target::Direct(Some(id_target)),
        });

        // On-attacked triggers respond to attack damage only
        if (from_card || from_monster) && id_source != Some(id_target) {
            let target = &mut state.entities[id_target];
            fire_on_damage_taken(
                target,
                id_target,
                damage_over_block,
                &mut state.effect_queue,
            );
        }
    }
}

fn fire_on_damage_taken(
    target: &mut Entity,
    id_target: usize,
    damage_over_block: u16,
    effect_queue: &mut VecDeque<Effect>,
) {
    // CurlUp, Flight and Malleable skip a killing blow
    let lives = damage_over_block < target.vitals.health;

    // CurlUp: fires once, gaining block = stacks behind the rest of the attack; the spent modifier stays until then
    if lives
        && has_modifier(&target.modifiers, ModifierKind::CurlUp)
        && !target.monster_curl_up_triggered
    {
        target.monster_curl_up_triggered = true;
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::CurlUp);
        effect_queue.push_back(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_target),
            target: Target::Direct(Some(id_target)),
        });
        effect_queue.push_back(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::CurlUp,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Angry: gain Strength = stacks every time it takes damage
    if has_modifier(&target.modifiers, ModifierKind::Angry) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::Angry);
        effect_queue.push_front(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Flight: a landing hit queues the stack loss behind the rest of the attack
    if lives && has_modifier(&target.modifiers, ModifierKind::Flight) {
        effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Flight,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Malleable: gain `stacks` block per hit taken, then escalate by one
    if lives && has_modifier(&target.modifiers, ModifierKind::Malleable) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::Malleable);
        modifier_apply(&mut target.modifiers, ModifierKind::Malleable, 1);
        effect_queue.push_back(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_target),
            target: Target::Direct(Some(id_target)),
        });
    }
}
