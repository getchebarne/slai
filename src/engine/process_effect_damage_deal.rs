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
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_damage_deal(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
) {
    let id_target = id_target.expect("DamageDeal requires id_target");

    // Get source entity type
    let from_card = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Card,
        None => false,
    };
    let from_monster = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Monster,
        None => false,
    };

    // Substract block and calculate damage over it
    let target = &mut state.entities[id_target];
    let block_prev = target.vitals.block;
    let mut damage_over_block = amount.saturating_sub(block_prev);
    target.vitals.block = block_prev.saturating_sub(amount);

    // Boot: the player's card hits leaving a 1-4 remainder land 5 instead
    if from_card
        && target.kind == EntityKind::Monster
        && 1 <= damage_over_block
        && damage_over_block <= 4
        && has_relic(&state.id_relics, RelicName::Boot)
    {
        damage_over_block = 5;
    }

    // Torii: monster hits leaving a 2-5 remainder on the character land 1 instead
    let id_character = state.id_character;
    if from_monster
        && id_target == id_character
        && 2 <= damage_over_block
        && damage_over_block <= 5
        && has_relic(&state.id_relics, RelicName::Torii)
    {
        damage_over_block = 1;
    }

    // Hand Drill: breaking a monster's block applies 2 Vulnerable
    if target.kind == EntityKind::Monster
        && block_prev > 0
        && target.vitals.block == 0
        && has_relic(&state.id_relics, RelicName::HandDrill)
    {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Executes in reverse:
    //     1. On-damage-taken triggers (CurlUp, Angry)
    //     2. ModifierGain Poison (Envenom)
    //     3. HealthDelta
    if damage_over_block > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(damage_over_block),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });

        // Envenom: card-played unblocked damage applies Poison; modifier-damage excluded
        let mods_char = state.entities[state.id_character].modifiers;
        if from_card && has_modifier(&mods_char, ModifierKind::Envenom) {
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
    if has_modifier(&target.modifiers, ModifierKind::CurlUp) {
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
    if has_modifier(&target.modifiers, ModifierKind::Angry) {
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
