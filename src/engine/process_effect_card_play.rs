use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::card_effective_cost;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_card_play(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardPlay requires id_target");
    let id_character = state.id_character;
    state.this_turn_cards_played = state.this_turn_cards_played.saturating_add(1);
    let this_turn_discards = state.this_turn_discards;
    let this_combat_damage_instances_taken = state.this_combat_damage_instances_taken;
    let energy_current = state.energy.energy_current;

    let card = state.entities[id_card];

    if card.card_kind == CardKind::Attack {
        state.this_turn_attacks = state.this_turn_attacks.saturating_add(1);

        if let Some(id_kunai) = state.id_relics[RelicName::Kunai as usize] {
            let counter = &mut state.entities[id_kunai].relic_counter;
            *counter += 1;
            if *counter >= 3 {
                *counter = 0;
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Dexterity,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
        if let Some(id_shuriken) = state.id_relics[RelicName::Shuriken as usize] {
            let counter = &mut state.entities[id_shuriken].relic_counter;
            *counter += 1;
            if *counter >= 3 {
                *counter = 0;
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Strength,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
    }

    let cost = card_effective_cost(
        &card,
        this_turn_discards,
        this_combat_damage_instances_taken,
        energy_current,
    );

    // X-cost reads raw energy_current so Setup-flagged X-cost still scales
    let multiplier = match card.card_cost_kind {
        CardCostKind::XCost { offset } => (energy_current as i16 + offset as i16).max(0) as usize,
        _ => 1,
    };

    if card.card_free_to_play_once {
        state.entities[id_card].card_free_to_play_once = false;
    }

    state.effect_buf.clear();

    state.effect_buf.push(Effect {
        kind: EffectKind::EnergyLoss { amount: cost },
        id_source: None,
        target: Target::Direct(None),
    });

    if card.card_exhaust {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardExhaust,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    } else if card.card_kind == CardKind::Power {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardRemove,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    } else {
        // Not a real discard: skips this_turn_discards and Reflex
        state.effect_buf.push(Effect {
            kind: EffectKind::CardMoveToDiscard,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    let char_modifiers = &state.entities[id_character].modifiers;

    if modifier_has(char_modifiers, ModifierKind::AfterImage) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::AfterImage);
        state.effect_buf.push(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    // ThousandCuts: id_source = None to skip Envenom proc and Strength/Weak scaling
    if modifier_has(char_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::ThousandCuts);
        for id_monster in state.id_monsters.iter().flatten().copied() {
            state.effect_buf.push(Effect {
                kind: EffectKind::DamageDeal {
                    amount: stacks as u16,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    if card.card_kind == CardKind::Attack {
        for id_monster in state.id_monsters.iter().flatten().copied() {
            let monster_modifiers = &state.entities[id_monster].modifiers;
            if modifier_has(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: stacks as u16,
                    },
                    id_source: Some(id_monster),
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
    }

    let char_modifiers = &state.entities[id_character].modifiers;
    // Burst (skill-only) doubles; X-cost multiplies by X; the two stack multiplicatively
    let burst =
        modifier_has(char_modifiers, ModifierKind::Burst) && card.card_kind == CardKind::Skill;
    let reps = if burst { 2 * multiplier } else { multiplier };
    for _ in 0..reps {
        for e in card.card_effects[..card.card_effects_len as usize].iter() {
            state.effect_buf.push(Effect {
                id_source: Some(id_card),
                ..*e
            });
        }
    }
    if burst {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Burst,
                stacks: -1,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    if card.card_kind == CardKind::Attack && modifier_has(char_modifiers, ModifierKind::Vigor) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Vigor,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Choke: pushed after card_effects so the played card resolves first
    for id_monster in state.id_monsters.iter().flatten().copied() {
        let mods_monster = &state.entities[id_monster].modifiers;
        if modifier_has(mods_monster, ModifierKind::Choke) {
            let stacks = modifier_stacks(mods_monster, ModifierKind::Choke);
            state.effect_buf.push(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Loss,
                    amount: Amount::Absolute(stacks as u16),
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // Enrage: gain strength on played skill
    if card.card_kind == CardKind::Skill {
        for id_monster in state.id_monsters.iter().flatten().copied() {
            let mods_monster = &state.entities[id_monster].modifiers;
            if modifier_has(mods_monster, ModifierKind::Enrage) {
                let stacks = modifier_stacks(mods_monster, ModifierKind::Enrage);
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Strength,
                        stacks,
                    },
                    id_source: Some(id_monster),
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }
    }

    // Pain: each copy in hand bleeds 1 HP on any other card play; HealthDelta ignores block
    for i in 0..state.id_hand.len() {
        if state.entities[state.id_hand[i]].card_name == CardName::Pain {
            state.effect_buf.push(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Loss,
                    amount: Amount::Absolute(1),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
    }

    flush_effects_from_buf_to_queue_front(state);
}
