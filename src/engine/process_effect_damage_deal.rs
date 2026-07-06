use std::collections::VecDeque;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::types::DeltaSign;
use crate::types::RelicName;

pub fn process_effect_damage_deal(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
) {
    let id_target = id_target.expect("DamageDeal requires id_target");
    let from_card = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Card,
        None => false,
    };
    let from_monster = match id_source {
        Some(id) => state.entities[id].kind == EntityKind::Monster,
        None => false,
    };
    let mods_char = state.entities[state.id_character].modifiers;
    let id_character = state.id_character;

    let target = &mut state.entities[id_target];
    let target_is_monster = target.kind == EntityKind::Monster;
    let block_before = target.vitals.block;
    let mut damage_over_block = amount.saturating_sub(block_before);
    target.vitals.block = block_before.saturating_sub(amount);
    let block_after = target.vitals.block;

    // Boot: the player's card hits leaving a 1-4 remainder land 5 instead
    if from_card
        && target_is_monster
        && (1..=4).contains(&damage_over_block)
        && state.id_relics[RelicName::Boot as usize].is_some()
    {
        damage_over_block = 5;
    }
    // Torii: monster hits leaving a 2-5 remainder on the character land 1 instead
    if from_monster
        && id_target == id_character
        && (2..=5).contains(&damage_over_block)
        && state.id_relics[RelicName::Torii as usize].is_some()
    {
        damage_over_block = 1;
    }
    // Hand Drill: breaking a monster's block applies 2 Vulnerable
    if target_is_monster
        && block_before > 0
        && block_after == 0
        && state.id_relics[RelicName::HandDrill as usize].is_some()
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
        if from_card && modifier_has(&mods_char, ModifierKind::Envenom) {
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
    if modifier_has(&target.modifiers, ModifierKind::CurlUp) {
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
    if modifier_has(&target.modifiers, ModifierKind::Angry) {
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

#[cfg(test)]
mod tests {
    use crate::engine::test_support::combat_with_relic;
    use crate::engine::test_support::end_turn;
    use crate::engine::test_support::first_monster;
    use crate::engine::test_support::play;
    use crate::engine::test_support::put_in_hand;
    use crate::modifier::ModifierKind;
    use crate::modifier::modifier_stacks;
    use crate::types::CardName;
    use crate::types::MonsterName;
    use crate::types::RelicName;

    #[test]
    fn boot_raises_small_remainders_to_five() {
        let mut state = combat_with_relic(RelicName::Boot, MonsterName::JawWorm);
        let id_monster = first_monster(&state);
        let hp_max = state.entities[id_monster].vitals.health_max;
        state.entities[id_monster].vitals.block = 3;
        // Strike 6 into 3 block: remainder 3 -> booted to 5
        let id = put_in_hand(&mut state, CardName::Strike);
        play(&mut state, id);
        assert_eq!(state.entities[id_monster].vitals.health, hp_max - 5);
        // Unblocked Strike: remainder 6 is out of the 1-4 window, unchanged
        let id = put_in_hand(&mut state, CardName::Strike);
        play(&mut state, id);
        assert_eq!(state.entities[id_monster].vitals.health, hp_max - 11);
    }

    #[test]
    fn torii_shrinks_small_incoming_hits_to_one() {
        let mut state = combat_with_relic(RelicName::Torii, MonsterName::JawWorm);
        let id_character = state.id_character;
        let hp_before = state.entities[id_character].vitals.health;
        state.entities[id_character].vitals.block = 7;
        // JawWorm opens Chomp 11: 11 - 7 = 4 remainder -> 1
        end_turn(&mut state);
        assert_eq!(state.entities[id_character].vitals.health, hp_before - 1);
    }

    #[test]
    fn hand_drill_applies_vulnerable_on_block_break() {
        let mut state = combat_with_relic(RelicName::HandDrill, MonsterName::JawWorm);
        let id_monster = first_monster(&state);
        state.entities[id_monster].vitals.block = 3;
        let id = put_in_hand(&mut state, CardName::Strike);
        play(&mut state, id);
        let mods = &state.entities[id_monster].modifiers;
        assert_eq!(modifier_stacks(mods, ModifierKind::Vulnerable), 2);
    }
}
