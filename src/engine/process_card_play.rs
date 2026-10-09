use rand::Rng;

use crate::consts::MAX_SIZE_HAND;
use crate::consts::PANACHE_PLAYS;
use crate::effect::Amount;
use crate::effect::CardPlay;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::PlaySource;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::relics::trigger_relic_counter;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Combat;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::detach_card;
use crate::utils::entity_requires_target;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::get_card_effective_cost;
use crate::utils::has_relic;
use crate::utils::is_play_restriction_satisfied;
use crate::utils::play_cap_reached;
use crate::utils::wrist_blade_bonus;

// Starts the next waiting Card play; a Replay plays the Card where it lies
pub fn process_card_play(state: &mut GameState, card_play: CardPlay) {
    let CardPlay {
        id_card,
        id_target,
        play_source,
        energy,
    } = card_play;

    assert!(
        state.combat.active,
        "process_card_play outside the Combat frame"
    );

    // Thinking Ahead puts back only if the hand held a Card as it was played, itself included;
    // Setup and Forethought check at the pick, and nothing refills the hand before it: same result
    let hand_nonempty_at_start = !state.combat.id_card_hand.is_empty();

    // A Card's reshuffle (Deep Breath) happens only if the discard pile held Cards as it was played
    let discard_nonempty_at_start = !state.combat.id_card_discard.is_empty();

    // A hand play leaves the hand up front; the Card stays pile-less until relocated
    if play_source == PlaySource::Hand {
        detach_card(&mut state.combat, id_card);
    }

    let Combat {
        id_card_hand,
        id_card_draw,
        id_card_exhaust,
        id_monsters,
        id_monster_picked,
        this_turn_attacks,
        this_turn_cards_played,
        panache_countdown,
        ..
    } = &mut state.combat;

    // Read-only here: copied out so the body below can borrow the whole state
    let id_character = state.id_character;
    let card = state.entities[id_card];

    // Another play of this Card still waits, such as a second replay
    let other_play_waits = state
        .card_play_queue
        .iter()
        .any(|card_play| card_play.id_card == id_card);

    // The last replay of an exhausted Card has read its cost this turn, which now drops
    if id_card_exhaust.contains(&id_card) && !other_play_waits {
        state.entities[id_card].card_cost_override = None;
    }

    // The Card's picked-Monster effects resolve against this play's target
    *id_monster_picked = id_target;

    // Draw-top plays and replays are re-gated when served; a Card needing a target needs it alive
    let target_gone = entity_requires_target(&card)
        && id_target.is_some_and(|id| !id_monsters.contains(&Some(id)));
    let entangled = has_modifier(
        &state.entities[id_character].modifiers,
        ModifierKind::Entangled,
    );
    let playable = is_play_restriction_satisfied(
        card.card_play_restriction,
        card.card_kind,
        id_card_draw,
        &state.id_relics,
    ) && !(entangled && card.card_kind == CardKind::Attack)
        && !target_gone
        && !play_cap_reached(
            id_card_hand,
            &state.entities,
            &state.id_relics,
            *this_turn_cards_played,
        );
    if !playable {
        // A refused play still spends free-to-play-once, unless another play of the Card waits
        if !other_play_waits {
            state.entities[id_card].card_free_to_play_once = false;
        }
        match play_source {
            PlaySource::Hand => unreachable!("a hand play is legal when served"),

            // Routed to its pile untriggered
            PlaySource::DrawTop => state.effect_queue.push_front(Effect {
                kind: EffectKind::CardPlayRelocate,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            }),

            // The replay fizzles: no counters, no hooks
            PlaySource::Replay => {}
        }
        return;
    }

    // Increase this-turn-played-Cards counter
    *this_turn_cards_played = this_turn_cards_played.saturating_add(1);

    // Pocketwatch and Velvet Choker count the turn's plays; the play cap stops Velvet Choker's at 6
    for name in [RelicName::Pocketwatch, RelicName::VelvetChoker] {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter += 1;
        }
    }

    if card.card_kind == CardKind::Attack {
        // Increase this-turn-played-attacks counter
        *this_turn_attacks = this_turn_attacks.saturating_add(1);
    }

    // On-power play triggers
    let mut urn_heal = false;
    let mut id_mummified = None;
    if card.card_kind == CardKind::Power {
        // Bird-Faced Urn: playing a Power heals 2
        urn_heal = has_relic(&state.id_relics, RelicName::BirdFacedUrn);

        // Mummified Hand: pick a random still-costed hand Card to make free this turn
        if has_relic(&state.id_relics, RelicName::MummifiedHand) {
            id_mummified = pick_random_costed_hand_card(
                &*id_card_hand,
                &state.entities,
                &mut state.rng,
                id_card,
                energy,
            );
        }
    }

    // Clear effect buffer — prepare it to be filled
    state.effect_buf.clear();

    // Mummified Hand's cut resolves first, so a replay of this play picks another Card
    if let Some(id_mummified) = id_mummified {
        state.effect_buf.push(Effect {
            kind: EffectKind::SetCostOverride {
                amount: 0,
                only_reduce: false,
                random: false,
                scope: CostScope::Turn,
            },
            id_source: None,
            target: Target::Direct(Some(id_mummified)),
        });
    }

    // Energy loss: every play reads its cost (Wrist Blade, Necronomicon), only a hand play pays it
    let cost_effective = get_card_effective_cost(&card, energy);
    if play_source == PlaySource::Hand {
        state.effect_buf.push(Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Loss,
                amount: cost_effective,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Pain: each copy in hand bleeds 1 HP on any other Card play
    for idx in 0..id_card_hand.len() {
        if state.entities[id_card_hand[idx]].card_name == CardName::Pain {
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

    // Enrage (enemy): a played Skill grants Strength ahead of the Skill's effects, behind Pain
    if card.card_kind == CardKind::Skill {
        for id_monster in id_monsters.iter().flatten().copied() {
            let mods_monster = &state.entities[id_monster].modifiers;
            if has_modifier(mods_monster, ModifierKind::Enrage) {
                let stacks = modifier_stacks(mods_monster, ModifierKind::Enrage);
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierDelta {
                        kind: ModifierKind::Strength,
                        stacks,
                    },
                    id_source: Some(id_monster),
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }
    }

    // Bird-Faced Urn's heal also runs ahead of the Card's effects, behind Pain
    if urn_heal {
        state.effect_buf.push(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(2),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Blue Candle / Medical Kit: a Relic-enabled play exhausts the Card for the rest of combat; the candle also costs 1 HP
    if card.card_kind == CardKind::Curse && has_relic(&state.id_relics, RelicName::BlueCandle) {
        state.effect_buf.push(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(1),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
        state.entities[id_card].card_exhaust = true;
    } else if card.card_kind == CardKind::Status
        && has_relic(&state.id_relics, RelicName::MedicalKit)
    {
        state.entities[id_card].card_exhaust = true;
    }

    // Necronomicon: the first Attack costing 2+ each turn is played twice
    let necronomicon = if play_source != PlaySource::Replay
        && card.card_kind == CardKind::Attack
        && cost_effective >= 2
        && let Some(id) = state.id_relics[RelicName::Necronomicon as usize]
        && state.entities[id].relic_counter == 0
    {
        state.entities[id].relic_counter = 1;
        true
    } else {
        false
    };

    let char_modifiers = &state.entities[id_character].modifiers;

    // Burst (skill-only) doubles; X-cost multiplies by X; they stack multiplicatively
    // X-cost reads `energy`, the X fixed when the play was queued
    let mul = match card.card_cost_kind {
        CardCostKind::XCost { offset } => {
            let x = (energy as i16 + offset as i16).max(0) as usize;

            // Chemical X: X+2 on effect reps; energy paid is unchanged
            if has_relic(&state.id_relics, RelicName::ChemicalX) {
                x + 2
            } else {
                x
            }
        }
        _ => 1,
    };
    let burst = play_source != PlaySource::Replay
        && has_modifier(char_modifiers, ModifierKind::Burst)
        && card.card_kind == CardKind::Skill;

    // DuplicateNextCardPlay replays any Card kind; additive with Burst
    let stacks_duplication = modifier_stacks(char_modifiers, ModifierKind::DuplicateNextCardPlay);
    let duplication = play_source != PlaySource::Replay && stacks_duplication > 0;

    // Burst and Duplication spend their stack before the Card's effects resolve
    if burst {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Burst,
                stacks: -1,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }
    if duplication {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::DuplicateNextCardPlay,
                stacks: -1,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    // Wrist Blade: attacks that cost 0 deal +4 per hit
    let bonus_wrist_blade = wrist_blade_bonus(&card, cost_effective, &state.id_relics);

    // X-cost repeats the effects inside the one play
    for effect in card.card_effects_play[..card.card_effects_play_len as usize].iter() {
        if matches!(effect.kind, EffectKind::CardSetupPick { .. }) && !hand_nonempty_at_start {
            continue;
        }
        if matches!(effect.kind, EffectKind::ShuffleDiscardPileIntoDrawPile)
            && !discard_nonempty_at_start
        {
            continue;
        }
        let mut effect = Effect {
            id_source: Some(id_card), // Stamp the Card's ID
            ..*effect
        };

        // Add Wrist Blade bonus to every hit of the Card's damage
        match &mut effect.kind {
            EffectKind::DamagePhysical { amount, .. }
            | EffectKind::DamagePhysicalIfPoisoned { amount } => *amount += bonus_wrist_blade,
            EffectKind::DamageFinisher { damage } | EffectKind::DamageFlechettes { damage } => {
                *damage += bonus_wrist_blade
            }
            EffectKind::DamageMindBlast { bonus } => *bonus += bonus_wrist_blade,
            _ => {}
        }

        // `EffectKind::ModifierDelta` scales its stacks into a single application whatever X is (e.g., Malaise)
        if let EffectKind::ModifierDelta { stacks, .. } = &mut effect.kind {
            *stacks *= mul as i16;
            state.effect_buf.push(effect);
            continue;
        }
        for _ in 0..mul {
            state.effect_buf.push(effect);
        }
    }

    // After Image: gain `stacks` block on Card play
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

    // Panache: every 5th Card played while active hits all enemies for `stacks`
    if has_modifier(char_modifiers, ModifierKind::Panache) {
        *panache_countdown -= 1;
        if *panache_countdown == 0 {
            *panache_countdown = PANACHE_PLAYS;
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

    // A spent DuplicateNextCardPlay stays at 0 stacks until the Card's effects have resolved
    if duplication && stacks_duplication == 1 {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::DuplicateNextCardPlay,
            },
            id_source: None,
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

    // Pen Nib: the doubled Attack spends the charge
    if card.card_kind == CardKind::Attack && has_modifier(char_modifiers, ModifierKind::PenNib) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::PenNib,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
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

    // On-use Relics count the play behind the Character's Modifier hooks, in pickup order
    let mut id_relics: Vec<usize> = state.id_relics.iter().flatten().copied().collect();
    id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);
    for id_relic in id_relics {
        match (state.entities[id_relic].relic_name, card.card_kind) {
            // Kunai, Shuriken, Ornamental Fan and Nunchaku count Attacks, Letter Opener Skills, Ink Bottle every Card
            (
                RelicName::Kunai
                | RelicName::Shuriken
                | RelicName::OrnamentalFan
                | RelicName::Nunchaku,
                CardKind::Attack,
            )
            | (RelicName::LetterOpener, CardKind::Skill)
            | (RelicName::InkBottle, _) => {
                if trigger_relic_counter(&mut state.entities[id_relic]) {
                    state
                        .effect_buf
                        .extend_from_slice(state.entities[id_relic].relic_effects_counter);
                }
            }

            // Pen Nib: every 10th Attack is doubled; the 9th primes the charge, the 10th starts the count over
            (RelicName::PenNib, CardKind::Attack) => {
                let counter = &mut state.entities[id_relic].relic_counter;
                *counter += 1;
                if *counter == 10 {
                    *counter = 0;
                } else if *counter == 9 {
                    state.effect_buf.push(Effect {
                        kind: EffectKind::ModifierDelta {
                            kind: ModifierKind::PenNib,
                            stacks: 1,
                        },
                        id_source: None,
                        target: Target::Direct(Some(id_character)),
                    });
                }
            }

            // Orange Pellets: Attack + Skill + Power in one turn sweeps all debuffs
            (RelicName::OrangePellets, _) => orange_pellets_track_and_sweep(
                &mut state.entities,
                &mut state.effect_buf,
                card.card_kind,
                id_relic,
                id_character,
            ),
            _ => {}
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
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
    }

    // Choke (enemy): pushed after card_effects_play so the played Card resolves first
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

    // Route the played Card to its pile once every on-use hook has resolved; a replay leaves it where it lies
    if play_source != PlaySource::Replay {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardPlayRelocate,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // Thousand Cuts: deal `stacks` damage to all Monsters once the played Card is routed
    let char_modifiers = &state.entities[id_character].modifiers;
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

    // Burst / Duplication / Necronomicon each replay the Card against the same target, ahead of
    // any play already waiting; changes to the Card (Glass Knife, Ritual Dagger) carry over
    let replays = burst as usize + duplication as usize + necronomicon as usize;
    for _ in 0..replays {
        state.card_play_queue.push_front(CardPlay {
            id_card,
            id_target,
            play_source: PlaySource::Replay,
            energy,
        });
    }

    // The Card's last waiting play spends free-to-play-once
    if replays == 0 && !other_play_waits {
        state.entities[id_card].card_free_to_play_once = false;
    }

    flush_effects_from_buf_to_queue_front(state);
}

// One random hand Card that still costs energy this turn
fn pick_random_costed_hand_card(
    id_card_hand: &[usize],
    entities: &[Entity],
    rng: &mut impl Rng,
    id_card_played: usize,
    energy_current: u16,
) -> Option<usize> {
    let mut cards_valid = [0usize; MAX_SIZE_HAND];
    let mut num = 0;
    for &id_card in id_card_hand.iter() {
        // Exclude just-played Card
        if id_card == id_card_played {
            continue;
        }

        // Calculate base and effective costs
        let card = &entities[id_card];
        let cost_base_positive =
            !matches!(card.card_cost_kind, CardCostKind::XCost { .. }) && card.card_cost > 0;
        let cost_effective = get_card_effective_cost(card, energy_current);

        // Only consider eligible if the base cost and effective costs are grater than zero (excludes X-cost)
        if cost_base_positive && cost_effective > 0 {
            cards_valid[num] = id_card;
            num += 1;
        }
    }

    // Sample
    if num > 0 {
        Some(cards_valid[rng.random_range(0..num)])
    } else {
        None
    }
}

// Tracks the played kind in a seen-kinds bitmask (Attack=1, Skill=2, Power=4) on the
// Relic counter; once all three are seen in a turn, clears the Character's debuffs and resets
fn orange_pellets_track_and_sweep(
    entities: &mut [Entity],
    effect_buf: &mut Vec<Effect>,
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
    effect_buf.push(Effect {
        kind: EffectKind::DebuffsClear,
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
}
