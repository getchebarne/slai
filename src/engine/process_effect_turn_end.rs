use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::relics::RELIC_COUNTERS_PER_TURN;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

pub fn process_effect_turn_end_monster(id_target: Option<usize>, state: &mut GameState) {
    let id_actor = id_target.expect("TurnEnd (monster) requires id_target");
    let modifiers = &state.entities[id_actor].modifiers;

    if has_modifier(modifiers, ModifierKind::Shackled) {
        let stacks = modifier_stacks(modifiers, ModifierKind::Shackled);
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
    if has_modifier(modifiers, ModifierKind::Metallicize) {
        let stacks = modifier_stacks(modifiers, ModifierKind::Metallicize);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_actor),
            target: Target::Direct(Some(id_actor)),
        });
    }

    let modifiers = &state.entities[id_actor].modifiers;
    if has_modifier(modifiers, ModifierKind::PlatedArmor) {
        let stacks = modifier_stacks(modifiers, ModifierKind::PlatedArmor);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_actor),
            target: Target::Direct(Some(id_actor)),
        });
    }
}

pub fn process_effect_turn_end_character(state: &mut GameState) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_turn_end_character outside Combat mode")
    };
    // Reset per-turn relic counters
    for &name in RELIC_COUNTERS_PER_TURN {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = 0;
        }
    }

    // Clear per-turn card cost overrides
    for entity in state.entities.iter_mut() {
        if matches!(entity.kind, EntityKind::Card) {
            entity.card_cost_override = None;
        }
    }

    // Clear effect buffer. Relic effects go through effect_buf so they
    // resolve before the monster turns
    state.effect_buf.clear();

    // Art of War: 1 energy next turn if no attacks were played this turn
    if combat.this_turn_attacks == 0 && has_relic(&state.id_relics, RelicName::ArtOfWar) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Pocketwatch: draw 3 extra cards next turn if 3 or fewer were played this turn
    if combat.this_turn_cards_played <= 3 && has_relic(&state.id_relics, RelicName::Pocketwatch) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DrawCardNextTurn,
                stacks: 3,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Orichalcum: character gains 6 block if it has none
    if state.entities[state.id_character].vitals.block == 0
        && has_relic(&state.id_relics, RelicName::Orichalcum)
    {
        state.effect_buf.push(Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Stone Calendar: 52 damage to all monsters at the end of turn 7; fires once, no reset
    if let Some(id) = state.id_relics[RelicName::StoneCalendar as usize] {
        let counter = &mut state.entities[id].relic_counter;
        *counter += 1;
        if *counter == 7 {
            for id_monster in combat.id_monsters.iter().flatten().copied() {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal { amount: 52 },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }
    }

    // Retain: pick up to `stacks` cards to keep through the end-of-turn discard
    let mods_char = &state.entities[state.id_character].modifiers;
    if has_modifier(mods_char, ModifierKind::Retain) && !combat.id_hand.is_empty() {
        let stacks = modifier_stacks(mods_char, ModifierKind::Retain);
        state.effect_buf.push(Effect {
            kind: EffectKind::CardRetain,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand {
                    filter: CandidatePoolCardFilter::Any,
                },
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
    for &id_card in &combat.id_hand {
        let card = &state.entities[id_card];
        match card.card_name {
            CardName::Burn => {
                let dmg_burn: u16 = if card.card_upgraded { 4 } else { 2 };
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal { amount: dmg_burn },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            CardName::Decay => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal { amount: 2 },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            CardName::Regret => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::HealthDelta {
                        sign: DeltaSign::Loss,
                        amount: Amount::Absolute(combat.id_hand.len() as u16),
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            _ => {}
        }
    }

    // Queue organic discards
    for &id_card in &combat.id_hand {
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
    for &id_card in &combat.id_hand {
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

    // Queue Monsters' turns
    for id_monster in combat.id_monsters.iter().flatten().copied() {
        state.effect_buf.push(Effect {
            kind: EffectKind::TurnStart,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
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
    if has_modifier(mods_char, ModifierKind::Burst) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Burst,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    if has_modifier(mods_char, ModifierKind::NoDraw) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::NoDraw,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    if has_modifier(mods_char, ModifierKind::Entangled) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Entangled,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Reset per-turn trackers in `GameState`
    combat.this_turn_discards = 0;
    combat.this_turn_attacks = 0;
    combat.this_turn_cards_played = 0;

    flush_effects_from_buf_to_queue_front(state);
}
