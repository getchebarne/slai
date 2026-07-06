use rand::Rng;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::card_effective_cost;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_is_buff;
use crate::modifier::modifier_kind_from_u8;
use crate::modifier::modifier_stacks;
use crate::relics::relic_counter_fire;
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

        if relic_counter_fire(RelicName::Kunai, 3, &state.id_relics, &mut state.entities) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Dexterity,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        if relic_counter_fire(RelicName::Shuriken, 3, &state.id_relics, &mut state.entities) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        // Relic-sourced block: id_source None skips Dex/Frail scaling
        if relic_counter_fire(RelicName::OrnamentalFan, 3, &state.id_relics, &mut state.entities) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::BlockGain { amount: 4 },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        if relic_counter_fire(RelicName::Nunchaku, 10, &state.id_relics, &mut state.entities) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::EnergyGain { amount: 1 },
                id_source: None,
                target: Target::Direct(None),
            });
        }
    }

    // Pen Nib: the 10th Attack resolves inside its own double-damage bracket
    let pen_nib_fires = card.card_kind == CardKind::Attack
        && relic_counter_fire(RelicName::PenNib, 10, &state.id_relics, &mut state.entities);

    // Thorns-type damage: id_source None skips Strength/Weak scaling and Envenom
    if card.card_kind == CardKind::Skill
        && relic_counter_fire(RelicName::LetterOpener, 3, &state.id_relics, &mut state.entities)
    {
        for id_monster in state.id_monsters.iter().flatten().copied() {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::DamageDeal { amount: 5 },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    if card.card_kind == CardKind::Power {
        if state.id_relics[RelicName::BirdFacedUrn as usize].is_some() {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(2),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        // Random hand card with base cost > 0 still costing > 0 becomes free this turn;
        // the played card is still in id_hand here, so exclude it
        if state.id_relics[RelicName::MummifiedHand as usize].is_some() {
            let mut eligible = [0usize; MAX_SIZE_HAND];
            let mut n = 0;
            for &id in &state.id_hand {
                if id == id_card {
                    continue;
                }
                let c = &state.entities[id];
                let base_positive =
                    !matches!(c.card_cost_kind, CardCostKind::XCost { .. }) && c.card_cost > 0;
                let effective = card_effective_cost(
                    c,
                    this_turn_discards,
                    this_combat_damage_instances_taken,
                    energy_current,
                );
                if base_positive && effective > 0 {
                    eligible[n] = id;
                    n += 1;
                }
            }
            if n > 0 {
                let id_pick = eligible[state.rng.random_range(0..n)];
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::SetCostOverride { amount: 0 },
                    id_source: None,
                    target: Target::Direct(Some(id_pick)),
                });
            }
        }
    }

    // Counts every card played; counter persists across turns and combats
    if relic_counter_fire(RelicName::InkBottle, 10, &state.id_relics, &mut state.entities) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // relic_counter is a seen-kinds bitmask; all three kinds in one turn sweeps debuffs
    if let Some(id_pellets) = state.id_relics[RelicName::OrangePellets as usize] {
        let bit = match card.card_kind {
            CardKind::Attack => 1,
            CardKind::Skill => 2,
            CardKind::Power => 4,
            _ => 0,
        };
        if bit != 0 {
            let counter = &mut state.entities[id_pellets].relic_counter;
            *counter |= bit;
            if *counter == 7 {
                *counter = 0;
                let mods = &state.entities[id_character].modifiers;
                let mut bits = mods.active;
                while bits != 0 {
                    let idx = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    let kind = modifier_kind_from_u8(idx as u8);
                    // Negative Strength/Dexterity count as debuffs despite is_buff
                    let negative_stat =
                        matches!(kind, ModifierKind::Strength | ModifierKind::Dexterity)
                            && mods.stacks[idx] < 0;
                    if !modifier_is_buff(kind) || negative_stat {
                        state.effect_queue.push_back(Effect {
                            kind: EffectKind::ModifierRemove { kind },
                            id_source: None,
                            target: Target::Direct(Some(id_character)),
                        });
                    }
                }
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
        CardCostKind::XCost { offset } => {
            let x = (energy_current as i16 + offset as i16).max(0) as usize;
            // Chemical X: X+2 on effect reps; energy paid is unchanged
            if state.id_relics[RelicName::ChemicalX as usize].is_some() {
                x + 2
            } else {
                x
            }
        }
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

    // Blue Candle / Medical Kit: relic-enabled plays exhaust; the candle also costs 1 HP
    let relic_exhaust = (card.card_kind == CardKind::Curse
        && state.id_relics[RelicName::BlueCandle as usize].is_some())
        || (card.card_kind == CardKind::Status
            && state.id_relics[RelicName::MedicalKit as usize].is_some());
    if card.card_kind == CardKind::Curse
        && state.id_relics[RelicName::BlueCandle as usize].is_some()
    {
        state.effect_buf.push(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(1),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    if card.card_exhaust || relic_exhaust {
        // Strange Spoon: on-play exhausts have a 50% chance to discard instead
        let spooned = state.id_relics[RelicName::StrangeSpoon as usize].is_some()
            && state.rng.random_range(0..100) < 50;
        state.effect_buf.push(Effect {
            kind: if spooned {
                EffectKind::CardMoveToDiscard
            } else {
                EffectKind::CardExhaust
            },
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
    if pen_nib_fires {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::PenNib,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }
    for _ in 0..reps {
        for e in card.card_effects[..card.card_effects_len as usize].iter() {
            state.effect_buf.push(Effect {
                id_source: Some(id_card),
                ..*e
            });
        }
    }
    if pen_nib_fires {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::PenNib,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
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
