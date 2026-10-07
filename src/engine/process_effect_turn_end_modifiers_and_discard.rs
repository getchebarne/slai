use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::shuffle;

// The Character's end-of-turn Modifiers, then the end-of-turn exhausts and discards, queued behind what its Relics and self-playing Cards left
pub fn process_effect_turn_end_modifiers_and_discard(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_turn_end_modifiers_and_discard outside the Combat frame"
    );
    let Combat {
        id_card_hand,
        bombs,
        ..
    } = &state.combat;
    let mods_char = &state.entities[state.id_character].modifiers;

    // Regeneration: heal `stacks`, then decrement by 1 (removed at 0); ahead of every other Modifier
    if has_modifier(mods_char, ModifierKind::Regeneration) {
        let stacks = modifier_stacks(mods_char, ModifierKind::Regeneration);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(stacks.max(0) as u16),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Regeneration,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Retain: pick up to `stacks` Cards to keep through the end-of-turn discard
    if has_modifier(mods_char, ModifierKind::Retain)
        && !id_card_hand.is_empty()
        // Runic Pyramid: keeps the whole hand
        && !has_relic(&state.id_relics, RelicName::RunicPyramid)
    {
        let stacks = modifier_stacks(mods_char, ModifierKind::Retain);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardRetain,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::InputUpTo {
                    count: stacks.max(0) as u16,
                },
            },
        });
    }

    // Ritual: gain `stacks` Strength each turn end; only Monsters skip the first tick
    if has_modifier(mods_char, ModifierKind::Ritual) {
        let stacks = modifier_stacks(mods_char, ModifierKind::Ritual);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Wraith Form: lose `stacks` Dexterity each turn end, ahead of LoseStrength and LoseDexterity
    if has_modifier(mods_char, ModifierKind::WraithForm) {
        let stacks = modifier_stacks(mods_char, ModifierKind::WraithForm);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Dexterity,
                stacks: -stacks,
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
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierGain {
                    kind: gain,
                    stacks: -stacks,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierRemove { kind: lose },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // Queue `EffectKind::ModifierRemove`s for Modifiers that clear at end of turn
    for kind in [
        ModifierKind::Burst,
        ModifierKind::NoDraw,
        ModifierKind::Entangled,
    ] {
        if has_modifier(mods_char, kind) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierRemove { kind },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // DuplicateNextCardPlay ticks down one stack; a last stack is removed, never left at 0
    if has_modifier(mods_char, ModifierKind::DuplicateNextCardPlay) {
        let effect_kind = if modifier_stacks(mods_char, ModifierKind::DuplicateNextCardPlay) > 1 {
            EffectKind::ModifierGain {
                kind: ModifierKind::DuplicateNextCardPlay,
                stacks: -1,
            }
        } else {
            EffectKind::ModifierRemove {
                kind: ModifierKind::DuplicateNextCardPlay,
            }
        };
        state.effect_queue.push_back(Effect {
            kind: effect_kind,
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Bombs count down when their slot comes, so the Retain pick still shows the old fuses
    if !bombs.is_empty() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::BombTick,
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Ethereal Cards exhaust first, in random order; Cards drawn after this point stay in hand
    let mut id_ethereal: Vec<usize> = id_card_hand
        .iter()
        .copied()
        .filter(|&id| state.entities[id].card_ethereal)
        .collect();
    shuffle(&mut id_ethereal, &mut state.rng);
    for id_card in id_ethereal {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardExhaust,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // The rest leave from the right, so the leftmost ends on top of the discard pile; Runic Pyramid keeps them
    if !has_relic(&state.id_relics, RelicName::RunicPyramid) {
        for &id_card in id_card_hand.iter().rev() {
            if state.entities[id_card].card_ethereal {
                continue;
            }
            state.effect_queue.push_back(Effect {
                kind: EffectKind::CardDiscard {
                    source: DiscardSource::EndOfTurn,
                },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }

    // The Monsters' turns wait for the exhausts and discards and what they set off (Dead Branch's Card)
    state.effect_queue.push_back(Effect {
        kind: EffectKind::MonsterTurns,
        id_source: None,
        target: Target::Direct(None),
    });
}
