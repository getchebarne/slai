use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::{DispatchResult, EffectBuf};
use crate::entity::Entity;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::types::Vitals;

pub fn process_effect_turn_end_monster(
    _vitals: &mut Vitals,
    modifiers: &Modifiers,
    id_actor: usize,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Refund negative Strength stacks from `Shackled`
    if modifier_has(modifiers, ModifierKind::Shackled) {
        let stacks = modifier_stacks(modifiers, ModifierKind::Shackled);
        queue.push_front(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Shackled,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
        queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    // Ritual: skip if newly applied.
    if modifier_has(modifiers, ModifierKind::Ritual)
        && !modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(modifiers, ModifierKind::Ritual);
        queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }
    DispatchResult::Continue
}

pub fn process_effect_turn_end_character(
    id_character: usize,
    entities: &[Entity],
    id_hand: &[usize],
    _card_target: Option<usize>,
    id_alive_monsters: &[usize],
    discards_this_turn: &mut u8,
    attacks_played_this_turn: &mut u8,
    _rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Reset per-turn counters at the boundary. Doing it synchronously here
    // (before the rest of the chain queues up) means SneakyStrike / Finisher
    // see a fresh 0 at the start of each player turn.
    *discards_this_turn = 0;
    *attacks_played_this_turn = 0;

    let character_modifiers = &entities[id_character].modifiers;
    // Stack locals
    let mut buf_effects = EffectBuf::new();

    if modifier_has(character_modifiers, ModifierKind::Retain) && !id_hand.is_empty() {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::Retain);
        buf_effects.push(Effect {
            kind: EffectKind::CardRetain,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input {
                    count: stacks as u8,
                },
            },
        });
    }

    if modifier_has(character_modifiers, ModifierKind::Ritual)
        && !character_modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::Ritual);
        buf_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    for &id_card in id_hand {
        buf_effects.push(Effect {
            kind: EffectKind::CardDiscardEndOfTurn,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    buf_effects.push(Effect {
        kind: EffectKind::ModifierSetNotNew,
        id_source: None,
        target: Target::Direct(None),
    });

    for &id_monster in id_alive_monsters {
        let monster = &entities[id_monster];
        buf_effects.push(Effect {
            kind: EffectKind::TurnStart,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });

        if let Some(move_idx) = monster.move_current {
            for e in monster.moves[move_idx].effects.iter() {
                buf_effects.push(Effect {
                    id_source: Some(id_monster),
                    ..*e
                });
            }
        }

        buf_effects.push(Effect {
            kind: EffectKind::MoveUpdate,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
        buf_effects.push(Effect {
            kind: EffectKind::TurnEnd,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    buf_effects.push(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });

    if modifier_has(character_modifiers, ModifierKind::Burst) {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Burst,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    buf_effects.push_all_front(queue);
    DispatchResult::Continue
}
