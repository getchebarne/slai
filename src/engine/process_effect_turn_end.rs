use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_turn_end_monster(id_target: Option<usize>, state: &mut GameState) {
    let id_actor = id_target.expect("TurnEnd (monster) requires id_target");
    let modifiers = &state.entities[id_actor].modifiers;

    if modifier_has(modifiers, ModifierKind::Shackled) {
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
    if modifier_has(modifiers, ModifierKind::Ritual)
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
    if modifier_has(modifiers, ModifierKind::Metallicize) {
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
    if modifier_has(modifiers, ModifierKind::PlatedArmor) {
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
    state.this_turn_discards = 0;
    state.this_turn_attacks = 0;
    state.this_turn_cards_played = 0;

    if let Some(id) = state.id_relics[RelicName::Kunai as usize] {
        state.entities[id].relic_counter = 0;
    }
    if let Some(id) = state.id_relics[RelicName::Shuriken as usize] {
        state.entities[id].relic_counter = 0;
    }

    for entity in state.entities.iter_mut() {
        if matches!(entity.kind, EntityKind::Card) {
            entity.card_cost_override = None;
        }
    }

    let id_character = state.id_character;
    let id_monsters = state.id_monsters;

    state.effect_buf.clear();

    let mods_char = &state.entities[id_character].modifiers;
    if modifier_has(mods_char, ModifierKind::Retain) && !state.id_hand.is_empty() {
        let stacks = modifier_stacks(mods_char, ModifierKind::Retain);
        state.effect_buf.push(Effect {
            kind: EffectKind::CardRetain,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                selection_kind: SelectionKind::Input {
                    count: stacks.max(0) as u16,
                },
            },
        });
    }

    if modifier_has(mods_char, ModifierKind::Ritual)
        && !mods_char.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(mods_char, ModifierKind::Ritual);
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    if modifier_has(mods_char, ModifierKind::PlatedArmor) {
        let stacks = modifier_stacks(mods_char, ModifierKind::PlatedArmor);
        state.effect_buf.push(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    if modifier_has(mods_char, ModifierKind::WraithForm) {
        let stacks = modifier_stacks(mods_char, ModifierKind::WraithForm);
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Dexterity,
                stacks: -stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    let hand_len = state.id_hand.len() as u16; // EOT hand size, before discard
    for &id_card in &state.id_hand {
        let card = &state.entities[id_card];
        match card.card_name {
            CardName::Burn => {
                let dmg: u16 = if card.card_upgraded { 4 } else { 2 };
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal { amount: dmg },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
            CardName::Decay => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal { amount: 2 },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
            CardName::Regret => {
                // each copy loses the full EOT hand size
                state.effect_buf.push(Effect {
                    kind: EffectKind::HealthDelta {
                        sign: DeltaSign::Loss,
                        amount: Amount::Fixed(hand_len),
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
            _ => {}
        }
    }

    for &id_card in &state.id_hand {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::EndOfTurn,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    state.effect_buf.push(Effect {
        kind: EffectKind::ModifierSetNotNew,
        id_source: None,
        target: Target::Direct(None),
    });

    // after ModifierSetNotNew so Weak/Frail keep is_new=true through next TurnStart tick
    for &id_card in &state.id_hand {
        match state.entities[id_card].card_name {
            CardName::Doubt => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Weak,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
            CardName::Shame => {
                state.effect_buf.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Frail,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
            _ => {}
        }
    }

    for id_monster in id_monsters.iter().flatten().copied() {
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
            kind: EffectKind::MoveUpdate,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::TurnEnd,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    state.effect_buf.push(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });

    let mods_char = &state.entities[id_character].modifiers;
    if modifier_has(mods_char, ModifierKind::Burst) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Burst,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    if modifier_has(mods_char, ModifierKind::NoDraw) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::NoDraw,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    if modifier_has(mods_char, ModifierKind::Entangled) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Entangled,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    flush_effects_from_buf_to_queue_front(state);
}
