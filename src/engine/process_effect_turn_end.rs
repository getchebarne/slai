use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::effect::effect_discover_pick;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::monsters::snake_plant;
use crate::relics::RELIC_COUNTERS_PER_TURN;
use crate::types::CardColor;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Combat;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

// The Character's turn end tears down the turn; Monsters unwind their per-turn kit
pub fn process_effect_turn_end(id_target: Option<usize>, state: &mut GameState) {
    let id_actor = id_target.expect("TurnEnd requires id_target");
    if id_actor == state.id_character {
        process_effect_turn_end_character(state);
    } else {
        process_effect_turn_end_monster(id_actor, state);
    }
}

fn process_effect_turn_end_monster(id_actor: usize, state: &mut GameState) {
    // Corpses don't unwind Shackled or gain Metallicize block
    if state.entities[id_actor].dead {
        return;
    }
    let modifiers = &state.entities[id_actor].modifiers;

    if has_modifier(modifiers, ModifierKind::Shackled) {
        let stacks = modifier_stacks(modifiers, ModifierKind::Shackled);
        // Executes in reverse:
        //     1. ModifierGain Strength
        //     2. ModifierRemove Shackled
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Shackled,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    let modifiers = &state.entities[id_actor].modifiers;
    if has_modifier(modifiers, ModifierKind::Ritual)
        && !modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(modifiers, ModifierKind::Ritual);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    let modifiers = &state.entities[id_actor].modifiers;
    for kind in [ModifierKind::Metallicize, ModifierKind::PlatedArmor] {
        if has_modifier(modifiers, kind) {
            let stacks = modifier_stacks(modifiers, kind);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::BlockGain {
                    amount: stacks as u16,
                },
                id_source: Some(id_actor),
                target: Target::Direct(Some(id_actor)),
            });
        }
    }

    // Malleable: per-hit escalation resets to base at the owner's turn end
    let modifiers = &mut state.entities[id_actor].modifiers;
    if has_modifier(modifiers, ModifierKind::Malleable) {
        modifiers.stacks[ModifierKind::Malleable as usize] = snake_plant::MALLEABLE_BASE;
    }
}

fn process_effect_turn_end_character(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_turn_end_character outside the Combat frame"
    );
    let Combat {
        id_card_hand,
        id_card_draw,
        id_card_discard,
        id_card_exhaust,
        id_card_stasis,
        id_monsters,
        this_turn_discards,
        this_turn_attacks,
        this_turn_cards_played,
        this_turn_panache,
        bomb_countdown,
        ..
    } = &mut state.combat;
    // Reset per-turn Relic counters
    for &name in RELIC_COUNTERS_PER_TURN {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = 0;
        }
    }

    // Clear per-turn Card cost overrides
    for id_card in id_card_hand
        .iter()
        .chain(id_card_draw.iter())
        .chain(id_card_discard.iter())
        .chain(id_card_exhaust.iter())
        .chain(id_card_stasis.iter().flatten())
    {
        let entity = &mut state.entities[*id_card];
        if matches!(
            entity.card_cost_override,
            Some(CostOverride {
                scope: CostScope::Turn,
                ..
            })
        ) {
            entity.card_cost_override = None;
        }
    }

    // Clear effect buffer. Relic effects go through effect_buf so they
    // resolve before the Monster turns
    state.effect_buf.clear();

    // Art of War: 1 energy next turn if no attacks were played this turn
    if *this_turn_attacks == 0 && has_relic(&state.id_relics, RelicName::ArtOfWar) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Pocketwatch: draw 3 extra Cards next turn if 3 or fewer were played this turn
    if *this_turn_cards_played <= 3 && has_relic(&state.id_relics, RelicName::Pocketwatch) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DrawCardNextTurn,
                stacks: 3,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Orichalcum: Character gains 6 block if it has none
    if state.entities[state.id_character].vitals.block == 0
        && has_relic(&state.id_relics, RelicName::Orichalcum)
    {
        state.effect_buf.push(Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Stone Calendar: 52 damage to all Monsters at the end of turn 7; fires once, no reset
    if let Some(id) = state.id_relics[RelicName::StoneCalendar as usize] {
        let counter = &mut state.entities[id].relic_counter;
        *counter += 1;
        if *counter == 7 {
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: 52,
                        lifesteal: false,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }
    }

    // Nilry's Codex: discover a Card, shuffled into a random draw-pile spot
    if has_relic(&state.id_relics, RelicName::NilrysCodex) {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Green,
                exclude: &[],
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        state
            .effect_buf
            .push(effect_discover_pick(None, CardPile::Draw));
    }

    // Retain: pick up to `stacks` Cards to keep through the end-of-turn discard
    let mods_char = &state.entities[state.id_character].modifiers;
    if has_modifier(mods_char, ModifierKind::Retain)
        && !id_card_hand.is_empty()
        // Runic Pyramid: keeps the whole hand
        && !has_relic(&state.id_relics, RelicName::RunicPyramid)
    {
        let stacks = modifier_stacks(mods_char, ModifierKind::Retain);
        state.effect_buf.push(Effect {
            kind: EffectKind::CardRetain,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Input {
                    count: stacks.max(0) as u16,
                },
            },
        });
    }

    // Ritual: gain `stacks` Strength each turn end, skipping the turn it was applied
    if has_modifier(mods_char, ModifierKind::Ritual)
        && !mods_char.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(mods_char, ModifierKind::Ritual);
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Plated Armor: gain block equal to stacks
    if has_modifier(mods_char, ModifierKind::PlatedArmor) {
        let stacks = modifier_stacks(mods_char, ModifierKind::PlatedArmor);
        state.effect_buf.push(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(state.id_character),
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Regeneration: heal `stacks`, then decrement by 1 (removed at 0)
    if has_modifier(mods_char, ModifierKind::Regeneration) {
        let stacks = modifier_stacks(mods_char, ModifierKind::Regeneration);
        state.effect_buf.push(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(stacks.max(0) as u16),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Regeneration,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Lose{Strength,Dexterity}: the borrowed stacks leave at turn end
    for (lose, gain) in [
        (ModifierKind::LoseStrength, ModifierKind::Strength),
        (ModifierKind::LoseDexterity, ModifierKind::Dexterity),
    ] {
        if has_modifier(mods_char, lose) {
            let stacks = modifier_stacks(mods_char, lose);
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierGain {
                    kind: gain,
                    stacks: -stacks,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierRemove { kind: lose },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // Wraith Form: lose `stacks` Dexterity each turn end
    if has_modifier(mods_char, ModifierKind::WraithForm) {
        let stacks = modifier_stacks(mods_char, ModifierKind::WraithForm);
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Dexterity,
                stacks: -stacks,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Card-held-in-hand-at-the-end-of-turn effects
    for &id_card in id_card_hand.iter() {
        let card = &state.entities[id_card];
        match card.card_name {
            CardName::Burn => {
                let dmg_burn: u16 = if card.card_upgraded { 4 } else { 2 };
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: dmg_burn,
                        lifesteal: false,
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            CardName::Decay => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: 2,
                        lifesteal: false,
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            CardName::Regret => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::HealthDelta {
                        sign: DeltaSign::Loss,
                        amount: Amount::Absolute(id_card_hand.len() as u16),
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            _ => {}
        }
    }

    // Queue organic discards
    for &id_card in id_card_hand.iter() {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::EndOfTurn,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // Queue not-new modifier set
    state.effect_buf.push(Effect {
        kind: EffectKind::ModifierSetNotNew,
        id_source: None,
        target: Target::Direct(None),
    });

    // After `ModifierSetNotNew` so Weak / Frail keep `is_new` through next
    // `EffectKind::TurnStart` tick
    for &id_card in id_card_hand.iter() {
        match state.entities[id_card].card_name {
            CardName::Doubt => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Weak,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            CardName::Shame => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Frail,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            _ => {}
        }
    }

    // Every Monster's turn start, then every Monster acts and rolls, then every turn end
    for id_monster in id_monsters.iter().flatten().copied() {
        state.effect_buf.push(Effect {
            kind: EffectKind::TurnStart,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }
    for id_monster in id_monsters.iter().flatten().copied() {
        state.effect_buf.push(Effect {
            kind: EffectKind::MoveExecute,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: None,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }
    for id_monster in id_monsters.iter().flatten().copied() {
        state.effect_buf.push(Effect {
            kind: EffectKind::TurnEnd,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    // Queue Character's turn start
    state.effect_buf.push(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });

    // Queue `EffectKind::ModifierRemove`s for Modifiers that clear at end of turn
    for kind in [
        ModifierKind::Burst,
        ModifierKind::NoDraw,
        ModifierKind::Entangled,
    ] {
        if has_modifier(mods_char, kind) {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierRemove { kind },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // The Bomb: lazily armed 3-turn timer, detonates for `stacks` on all enemies
    if has_modifier(mods_char, ModifierKind::TheBomb) {
        if *bomb_countdown == 0 {
            *bomb_countdown = 3;
        }
        *bomb_countdown -= 1;
        if *bomb_countdown == 0 {
            let stacks = modifier_stacks(mods_char, ModifierKind::TheBomb);
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
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierRemove {
                    kind: ModifierKind::TheBomb,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // Reset per-turn trackers
    *this_turn_discards = 0;
    *this_turn_attacks = 0;
    *this_turn_cards_played = 0;
    *this_turn_panache = 0;

    flush_effects_from_buf_to_queue_front(state);
}
