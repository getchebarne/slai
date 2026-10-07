use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_set_not_new;
use crate::modifier::modifier_stacks;
use crate::monsters::snake_plant;
use crate::relics::RELIC_COUNTERS_PER_TURN;
use crate::relics::iter_owned_relics;
use crate::types::CardName;
use crate::types::Combat;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

// The Character's turn end runs its Relics and self-playing Cards; Monsters unwind their per-turn kit
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
        this_turn_discards,
        this_turn_panache,
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

    // What the turn applied stops being new before Doubt and Shame land; MonsterTurns re-clears Vulnerable
    modifier_set_not_new(&mut state.entities[state.id_character].modifiers);

    // Orichalcum: Character gains 6 block if it has none, ahead of the other turn-end Relics
    if state.entities[state.id_character].vitals.block == 0
        && has_relic(&state.id_relics, RelicName::Orichalcum)
    {
        state.effect_buf.push(Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Turn-end Relic effects, in acquisition order (Stone Calendar's damage, Nilry's Codex discover)
    let mut id_relics: Vec<usize> = iter_owned_relics(&state.id_relics)
        .map(|(_, id)| id)
        .collect();
    id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);

    for id_relic in id_relics {
        let relic = &mut state.entities[id_relic];

        // Stone Calendar: fires once at the reset threshold (end of turn 7), no reset
        if relic.relic_name == RelicName::StoneCalendar {
            relic.relic_counter += 1;
            if relic.relic_counter == relic.relic_counter_reset {
                for &effect in relic.relic_effects_counter {
                    state.effect_buf.push(effect);
                }
            }
        }
        for &effect in relic.relic_effects_turn_end {
            state.effect_buf.push(effect);
        }
    }

    let mods_char = &state.entities[state.id_character].modifiers;

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

    // Burn / Decay / Regret / Doubt / Shame play themselves out of hand
    for &id_card in id_card_hand.iter() {
        let card = &state.entities[id_card];
        match card.card_name {
            CardName::Burn => {
                let damage: u16 = if card.card_upgraded { 4 } else { 2 };
                state.effect_buf.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: damage,
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
            _ => continue,
        }
        state.effect_buf.push(Effect {
            kind: EffectKind::CardPlayRelocate {
                exhaust: card.card_exhaust,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // The end-of-turn Modifiers and the discard wait for the Cards above
    state.effect_buf.push(Effect {
        kind: EffectKind::TurnEndModifiersAndDiscard,
        id_source: None,
        target: Target::Direct(None),
    });

    // Reset per-turn trackers; attacks and plays count on until the next turn start
    *this_turn_discards = 0;
    *this_turn_panache = 0;

    flush_effects_from_buf_to_queue_front(state);
}
