use std::collections::VecDeque;

use rand::Rng;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::entity::Entity;
use crate::entity::get_card_effective_cost;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::relics::trigger_relic_counter;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::detach_card;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::mode_top_mut;

pub fn process_effect_card_play(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardPlay requires id_target");

    // Detach the played Card up front; it stays pile-less until its effects resolve
    detach_card(mode_top_mut(&mut state.mode_stack), id_card);

    let Mode::Combat {
        id_hand,
        id_monsters,
        energy,
        this_turn_discards,
        this_turn_attacks,
        this_turn_cards_played,
        this_turn_panache,
        this_combat_damage_instances_taken,
        ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("process_effect_card_play outside Combat mode")
    };

    // Read-only here: copied out so the body below can borrow the whole state
    let id_character = state.id_character;
    let this_turn_discards = *this_turn_discards;
    let this_combat_damage_instances_taken = *this_combat_damage_instances_taken;
    let energy_current = energy.energy_current;
    let card = state.entities[id_card];

    // Increase this-turn-played-Cards counter
    *this_turn_cards_played = this_turn_cards_played.saturating_add(1);

    if card.card_kind == CardKind::Attack {
        // Increase this-turn-played-attacks counter
        *this_turn_attacks = this_turn_attacks.saturating_add(1);

        // Kunai: every 3 Attacks played grants 1 Dexterity
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

        // Shuriken: every 3 Attacks played grants 1 Strength
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

        // Nunchaku: every 10 Attacks played grants 1 energy
        if trigger_relic_counter(
            RelicName::Nunchaku,
            10,
            &state.id_relics,
            &mut state.entities,
        ) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::EnergyDelta {
                    sign: DeltaSign::Gain,
                    amount: 1,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Pen Nib: every 10th Attack is doubled; 9 primes the charge, 10 consumes it
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

    // Letter Opener: every 3 Skills played deals 5 damage to all Monsters
    if card.card_kind == CardKind::Skill
        && trigger_relic_counter(
            RelicName::LetterOpener,
            3,
            &state.id_relics,
            &mut state.entities,
        )
    {
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::DamageDeal {
                    amount: 5,
                    lifesteal: false,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // On-power play triggers
    if card.card_kind == CardKind::Power {
        // Bird-Faced Urn: playing a Power heals 2
        if has_relic(&state.id_relics, RelicName::BirdFacedUrn) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(2),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        // Mummified Hand: make a random still-costed hand Card free this turn
        if has_relic(&state.id_relics, RelicName::MummifiedHand) {
            free_random_costed_hand_card(
                &*id_hand,
                &state.entities,
                &mut state.rng,
                &mut state.effect_queue,
                id_card,
                this_turn_discards,
                this_combat_damage_instances_taken,
                energy_current,
            );
        }
    }

    // Ink Bottle: Counts every Card played; counter persists across turns and combats
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
    if let Some(id_relic_pellets) = state.id_relics[RelicName::OrangePellets as usize] {
        orange_pellets_track_and_sweep(
            &mut state.entities,
            &mut state.effect_queue,
            card.card_kind,
            id_relic_pellets,
            id_character,
        );
    }

    // Until-played overrides are consumed by this play
    if matches!(
        card.card_cost_override,
        Some(CostOverride {
            scope: CostScope::UntilPlayed,
            ..
        })
    ) {
        state.entities[id_card].card_cost_override = None;
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
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Loss,
            amount: (effective_cost) as u16,
        },
        id_source: None,
        target: Target::Direct(None),
    });

    // Blue Candle / Medical Kit: Relic-enabled plays exhaust; the candle also costs 1 HP
    let relic_exhaust = if card.card_kind == CardKind::Curse
        && has_relic(&state.id_relics, RelicName::BlueCandle)
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
        && has_relic(&state.id_relics, RelicName::MedicalKit)
    {
        true
    } else {
        false
    };

    let char_modifiers = &state.entities[id_character].modifiers;

    // Burst (skill-only) doubles; X-cost multiplies by X; they stack multiplicatively
    // X-cost reads raw energy_current so Setup-flagged X-cost still scales
    let mul = match card.card_cost_kind {
        CardCostKind::XCost { offset } => {
            let x = (energy_current as i16 + offset as i16).max(0) as usize;
            // Chemical X: X+2 on effect reps; energy paid is unchanged
            if has_relic(&state.id_relics, RelicName::ChemicalX) {
                x + 2
            } else {
                x
            }
        }
        _ => 1,
    };
    let burst =
        has_modifier(char_modifiers, ModifierKind::Burst) && card.card_kind == CardKind::Skill;

    // DuplicateNextCardPlay replays any Card kind; additive with Burst
    let duplication = has_modifier(char_modifiers, ModifierKind::DuplicateNextCardPlay);

    // Total amount of Card-play repetitions
    let reps = (1 + burst as usize + duplication as usize) * mul;

    // Wrist Blade: attacks that cost 0 deal +4 per hit
    let wrist_blade_bonus = effective_cost == 0
        && card.card_kind == CardKind::Attack
        && !matches!(card.card_cost_kind, CardCostKind::XCost { .. }) // X-cost never qualifies
        && has_relic(&state.id_relics, RelicName::WristBlade);

    // Push the Card's on-play effects once for each rep
    for _ in 0..reps {
        for e in card.card_effects[..card.card_effects_len as usize].iter() {
            let mut effect = Effect {
                id_source: Some(id_card), // Stamp the Card's ID
                ..*e
            };

            // Add Wrist Blade bonus
            if wrist_blade_bonus && let EffectKind::DamagePhysical { amount, .. } = &mut effect.kind
            {
                *amount += 4;
            }

            // Push
            state.effect_buf.push(effect);
        }
    }

    // Relocation and on-play triggers land after the Card's own effects
    if card.card_exhaust || relic_exhaust {
        // Strange Spoon: on-play exhausts have a 50% chance to discard instead; Powers exempt
        let effect_kind = if card.card_kind != CardKind::Power
            && has_relic(&state.id_relics, RelicName::StrangeSpoon)
            && state.rng.random_range(0..100) < 50
        {
            EffectKind::CardMove {
                pile: CardPile::Discard,
                cost_zero: None,
            }
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
            kind: EffectKind::CardMove {
                pile: CardPile::Discard,
                cost_zero: None,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // After Image
    if has_modifier(char_modifiers, ModifierKind::AfterImage) {
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
    if has_modifier(char_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::ThousandCuts);
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_buf.push(Effect {
                kind: EffectKind::DamageDeal {
                    amount: stacks as u16,
                    lifesteal: false,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // Panache: every 5th Card played while active hits all enemies for `stacks`
    if has_modifier(char_modifiers, ModifierKind::Panache) {
        *this_turn_panache += 1;
        if *this_turn_panache == 5 {
            *this_turn_panache = 0;
            let stacks = modifier_stacks(char_modifiers, ModifierKind::Panache);
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: stacks.max(0) as u16,
                        lifesteal: false,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }
    }

    // Sharp Hide (enemy)
    if card.card_kind == CardKind::Attack {
        for id_monster in id_monsters.iter().flatten().copied() {
            let monster_modifiers = &state.entities[id_monster].modifiers;
            if has_modifier(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: stacks as u16,
                        lifesteal: false,
                    },
                    id_source: Some(id_monster),
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
    }

    // Consume 1 Burst and DuplicateNextCardPlay stack
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
    if duplication {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DuplicateNextCardPlay,
                stacks: -1,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    // Vigor
    if card.card_kind == CardKind::Attack && has_modifier(char_modifiers, ModifierKind::Vigor) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Vigor,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Choke (enemy): pushed after card_effects so the played Card resolves first
    for id_monster in id_monsters.iter().flatten().copied() {
        let mods_monster = &state.entities[id_monster].modifiers;
        if has_modifier(mods_monster, ModifierKind::Choke) {
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
        for id_monster in id_monsters.iter().flatten().copied() {
            let mods_monster = &state.entities[id_monster].modifiers;
            if has_modifier(mods_monster, ModifierKind::Enrage) {
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

    // Hex: playing a non-Attack shuffles Dazed into the draw pile
    if card.card_kind != CardKind::Attack && has_modifier(char_modifiers, ModifierKind::Hex) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::Hex);
        state.effect_buf.push(Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Dazed,
                pile: CardPile::Draw,
                count: stacks.max(0) as u16,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Pain: each copy in hand bleeds 1 HP on any other Card play; HealthDelta ignores block
    for i in 0..id_hand.len() {
        if state.entities[id_hand[i]].card_name == CardName::Pain {
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

// Zeroes the cost of one random hand Card that still costs energy this turn
fn free_random_costed_hand_card(
    id_hand: &[usize],
    entities: &[Entity],
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
    id_card_played: usize,
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
) {
    let mut cards_valid = [0usize; MAX_SIZE_HAND];
    let mut num = 0;
    for &id_card in id_hand.iter() {
        // Exclude just-played Card
        if id_card == id_card_played {
            continue;
        }

        // Calculate base and effective costs
        let card = &entities[id_card];
        let cost_base_positive =
            !matches!(card.card_cost_kind, CardCostKind::XCost { .. }) && card.card_cost > 0;
        let cost_effective = get_card_effective_cost(
            card,
            this_turn_discards,
            this_combat_damage_instances_taken,
            energy_current,
        );

        // Only consider eligible if the base cost and effective costs are grater than zero (excludes X-cost)
        if cost_base_positive && cost_effective > 0 {
            cards_valid[num] = id_card;
            num += 1;
        }
    }

    // Sample
    if num > 0 {
        let id_pick = cards_valid[rng.random_range(0..num)];
        effect_queue.push_back(Effect {
            kind: EffectKind::SetCostOverride {
                amount: 0,
                only_reduce: false,
                random: false,
                scope: CostScope::Turn,
            },
            id_source: None,
            target: Target::Direct(Some(id_pick)),
        });
    }
}

// Tracks the played kind in a seen-kinds bitmask (Attack=1, Skill=2, Power=4) on the
// Relic counter; once all three are seen in a turn, clears the character's debuffs and resets
fn orange_pellets_track_and_sweep(
    entities: &mut [Entity],
    effect_queue: &mut VecDeque<Effect>,
    card_kind: CardKind,
    id_relic_pellets: usize,
    id_character: usize,
) {
    // Get bit
    let bit = match card_kind {
        CardKind::Attack => 1,
        CardKind::Skill => 2,
        CardKind::Power => 4,
        _ => return,
    };

    // Increase `relic_counter`
    let counter = &mut entities[id_relic_pellets].relic_counter;
    *counter |= bit;

    // If all three types (Attack, Skill, Power) have not been played yet, return
    if *counter != 7 {
        return;
    }

    // Else, reset the counter and queue the debuff sweep
    *counter = 0;
    effect_queue.push_back(Effect {
        kind: EffectKind::DebuffsClear,
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
}
