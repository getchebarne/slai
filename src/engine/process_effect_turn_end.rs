use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
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
    _rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let character_modifiers = &entities[id_character].modifiers;
    // Stack locals
    let mut buf_effects = EffectBuf::new();

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
            kind: EffectKind::CardDiscard,
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
