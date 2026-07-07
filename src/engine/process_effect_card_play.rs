use rand::Rng;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::get_card_effective_cost;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_is_buff;
use crate::modifier::modifier_kind_from_u8;
use crate::modifier::modifier_stacks;
use crate::relics::trigger_relic_counter;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_card_play(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardPlay requires id_target");
    let id_character = state.id_character;
    let this_turn_discards = state.this_turn_discards;
    let this_combat_damage_instances_taken = state.this_combat_damage_instances_taken;
    let energy_current = state.energy.energy_current;
    let card = state.entities[id_card];

    // Increase this-turn-played-cards counter
    state.this_turn_cards_played = state.this_turn_cards_played.saturating_add(1);

    if card.card_kind == CardKind::Attack {
        // Increase this-turn-played-attacks counter
        state.this_turn_attacks = state.this_turn_attacks.saturating_add(1);

        // Kunai
        if trigger_relic_counter(RelicName::Kunai, 3, &state.id_relics, &mut state.entities) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Dexterity,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }

        // Shuriken
        if trigger_relic_counter(
            RelicName::Shuriken,
            3,
            &state.id_relics,
            &mut state.entities,
        ) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }

        // Ornamental Fan: id_source=None skips Dex / Frail scaling
        if trigger_relic_counter(
            RelicName::OrnamentalFan,
            3,
            &state.id_relics,
            &mut state.entities,
        ) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::BlockGain { amount: 4 },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }

        // Nunchaku
        if trigger_relic_counter(
            RelicName::Nunchaku,
            10,
            &state.id_relics,
            &mut state.entities,
        ) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::EnergyGain { amount: 1 },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Pen Nib
        if let Some(id_pen_nib) = state.id_relics[RelicName::PenNib as usize] {
            let counter = &mut state.entities[id_pen_nib].relic_counter;
            *counter += 1;
            match *counter {
                // Consumed: this attack was doubled by the live charge. Remove it
                10 => {
                    *counter = 0;
                    state.effect_queue.push_back(Effect {
                        kind: EffectKind::ModifierRemove {
                            kind: ModifierKind::PenNib,
                        },
                        id_source: None,
                        target: Target::Direct(Some(id_character)),
                    });
                }

                // Prime the next attack
                9 => state.effect_queue.push_back(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::PenNib,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                }),
                _ => {}
            }
        }
    }

    // Letter Opener
    if card.card_kind == CardKind::Skill
        && trigger_relic_counter(
            RelicName::LetterOpener,
            3,
            &state.id_relics,
            &mut state.entities,
        )
    {
        for id_monster in state.id_monsters.iter().flatten().copied() {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::DamageDeal { amount: 5 },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // On-power play triggers
    if card.card_kind == CardKind::Power {
        // Bird-Faced Urn
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
        // Mummified Hand: make a random still-costed hand card free this turn
        if state.id_relics[RelicName::MummifiedHand as usize].is_some() {
            free_random_costed_hand_card(
                state,
                id_card,
                this_turn_discards,
                this_combat_damage_instances_taken,
                energy_current,
            );
        }
    }

    // Ink Bottle: Counts every card played; counter persists across turns and combats
    if trigger_relic_counter(
        RelicName::InkBottle,
        10,
        &state.id_relics,
        &mut state.entities,
    ) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Orange Pellets: Attack + Skill + Power in one turn sweeps all debuffs
    if let Some(id_pellets) = state.id_relics[RelicName::OrangePellets as usize] {
        orange_pellets_track_and_sweep(state, id_pellets, card.card_kind, id_character);
    }

    // Clear `free_to_play_once` flag
    if card.card_free_to_play_once {
        state.entities[id_card].card_free_to_play_once = false;
    }

    // Clear effect buffer — prepare it to be filled
    state.effect_buf.clear();

    // Energy loss
    let effective_cost = get_card_effective_cost(
        &card,
        this_turn_discards,
        this_combat_damage_instances_taken,
        energy_current,
    );
    state.effect_buf.push(Effect {
        kind: EffectKind::EnergyLoss {
            amount: effective_cost,
        },
        id_source: None,
        target: Target::Direct(None),
    });

    // Blue Candle / Medical Kit: relic-enabled plays exhaust; the candle also costs 1 HP
    let relic_exhaust = if card.card_kind == CardKind::Curse
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
        true
    } else if card.card_kind == CardKind::Status
        && state.id_relics[RelicName::MedicalKit as usize].is_some()
    {
        true
    } else {
        false
    };

    // Relocate the card to the appropriate pile
    if card.card_exhaust || relic_exhaust {
        // Strange Spoon: on-play exhausts have a 50% chance to discard instead
        let effect_kind = if state.id_relics[RelicName::StrangeSpoon as usize].is_some()
            && state.rng.random_range(0..100) < 50
        {
            EffectKind::CardMoveToDiscard
        } else {
            EffectKind::CardExhaust
        };
        state.effect_buf.push(Effect {
            kind: effect_kind,
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
        // Not a real discard: skips this_turn_discards and on discard triggers
        state.effect_buf.push(Effect {
            kind: EffectKind::CardMoveToDiscard,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // Modifier triggers
    let char_modifiers = &state.entities[id_character].modifiers;

    // After Image
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

    // Thousand Cuts
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

    // Sharp Hide (enemy)
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

    // Burst (skill-only) doubles; X-cost multiplies by X; the two stack multiplicatively
    // X-cost reads raw energy_current so Setup-flagged X-cost still scales
    let mul = match card.card_cost_kind {
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
    let burst =
        modifier_has(char_modifiers, ModifierKind::Burst) && card.card_kind == CardKind::Skill;
    let reps = if burst { 2 * mul } else { mul };
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

    // Vigor
    if card.card_kind == CardKind::Attack && modifier_has(char_modifiers, ModifierKind::Vigor) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Vigor,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Choke (enemy): pushed after card_effects so the played card resolves first
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

    // Enrage (enemy): gain strength on played skill
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

// Zeroes the cost of one random hand card that still costs energy this turn.
// Excludes the just-played card (still in id_hand here), X-cost, and base-0 cards.
fn free_random_costed_hand_card(
    state: &mut GameState,
    id_played: usize,
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
) {
    let mut eligible = [0usize; MAX_SIZE_HAND];
    let mut n = 0;
    for &id in &state.id_hand {
        if id == id_played {
            continue;
        }
        let c = &state.entities[id];
        let base_positive =
            !matches!(c.card_cost_kind, CardCostKind::XCost { .. }) && c.card_cost > 0;
        let effective = get_card_effective_cost(
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

// Tracks the played kind in a seen-kinds bitmask (Attack=1, Skill=2, Power=4) on the
// relic counter; once all three are seen in a turn, clears the character's debuffs and resets.
fn orange_pellets_track_and_sweep(
    state: &mut GameState,
    id_pellets: usize,
    card_kind: CardKind,
    id_character: usize,
) {
    let bit = match card_kind {
        CardKind::Attack => 1,
        CardKind::Skill => 2,
        CardKind::Power => 4,
        _ => return,
    };
    let counter = &mut state.entities[id_pellets].relic_counter;
    *counter |= bit;
    if *counter != 7 {
        return;
    }
    *counter = 0;

    let mods = &state.entities[id_character].modifiers;
    let mut bits = mods.active;
    while bits != 0 {
        let idx = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        let kind = modifier_kind_from_u8(idx as u8);
        // Negative Strength/Dexterity count as debuffs despite is_buff
        let negative_stat = matches!(kind, ModifierKind::Strength | ModifierKind::Dexterity)
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
