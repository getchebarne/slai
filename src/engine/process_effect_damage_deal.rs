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
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::monsters::byrd;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::MonsterName;
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

    // Substract block and calculate damage over it
    let target = &mut state.entities[id_target];
    let block_prev = target.vitals.block;
    let health_prev = target.vitals.health;
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
    //     1. On-damage-taken triggers (CurlUp, Angry, Flight, Malleable)
    //     2. ModifierGain Poison (Envenom)
    //     3. HealthDelta
    //     4. HealthDelta Gain (lifesteal)
    if damage_over_block > 0 {
        // Life Suck: the source drinks the HP the target actually loses
        if lifesteal && let Some(id_src) = id_source {
            let heal = damage_over_block.min(health_prev);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(heal),
                },
                id_source: None,
                target: Target::Direct(Some(id_src)),
            });
        }

        state.effect_queue.push_front(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(damage_over_block),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });

        // Envenom: Card-played unblocked damage applies Poison; modifier-damage excluded
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

        // Painful Stabs: each unblocked hit from the owner adds a Wound to the discard pile
        if from_monster
            && id_target == id_character
            && let Some(id_src) = id_source
            && has_modifier(
                &state.entities[id_src].modifiers,
                ModifierKind::PainfulStabs,
            )
        {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Wound,
                    pile: CardPile::Discard,
                    count: 1,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // On-attacked triggers respond to attack damage only
        if (from_card || from_monster) && id_source != Some(id_target) {
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

    // Flight: each landing hit removes a stack; at zero the flier is grounded and stunned
    if has_modifier(&target.modifiers, ModifierKind::Flight) {
        modifier_apply(&mut target.modifiers, ModifierKind::Flight, -1);
        if !has_modifier(&target.modifiers, ModifierKind::Flight) {
            let idx_stunned = match target.monster_name {
                MonsterName::Byrd => byrd::IDX_MOVE_STUNNED,
                _ => panic!("Flight on unexpected monster: {:?}", target.monster_name),
            };
            effect_queue.push_front(Effect {
                kind: EffectKind::MoveUpdate {
                    move_override: Some(idx_stunned),
                },
                id_source: None,
                target: Target::Direct(Some(id_target)),
            });
        }
    }

    // Malleable: gain `stacks` block per hit taken, then escalate by one
    if has_modifier(&target.modifiers, ModifierKind::Malleable) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::Malleable);
        modifier_apply(&mut target.modifiers, ModifierKind::Malleable, 1);
        effect_queue.push_front(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_target),
            target: Target::Direct(Some(id_target)),
        });
    }
}
