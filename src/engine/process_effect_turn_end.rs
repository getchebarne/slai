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
    actor: usize,
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
            source: None,
            target: Target::Direct(Some(actor)),
        });
    }
    DispatchResult::Continue
}

pub fn process_effect_turn_end_character(
    character: usize,
    entities: &[Entity],
    hand: &[usize],
    _card_target: Option<usize>,
    alive_monsters: &[usize],
    _rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let character_modifiers = &entities[character].modifiers;
    let mut top = EffectBuf::new();

    if modifier_has(character_modifiers, ModifierKind::Ritual)
        && !character_modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::Ritual);
        top.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            source: None,
            target: Target::Direct(Some(character)),
        });
    }

    for &id_card in hand {
        top.push(Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    top.push(Effect {
        kind: EffectKind::ModifierSetNotNew,
        source: None,
        target: Target::Direct(None),
    });

    for &mid in alive_monsters {
        let monster = &entities[mid];
        top.push(Effect {
            kind: EffectKind::TurnStart,
            source: None,
            target: Target::Direct(Some(mid)),
        });

        if let Some(move_idx) = monster.move_current {
            for e in monster.moves[move_idx].effects.iter() {
                top.push(Effect {
                    source: Some(mid),
                    ..*e
                });
            }
        }

        top.push(Effect {
            kind: EffectKind::MoveUpdate,
            source: None,
            target: Target::Direct(Some(mid)),
        });
        top.push(Effect {
            kind: EffectKind::TurnEnd,
            source: None,
            target: Target::Direct(Some(mid)),
        });
    }

    top.push(Effect {
        kind: EffectKind::TurnStart,
        source: None,
        target: Target::Direct(Some(character)),
    });

    if modifier_has(character_modifiers, ModifierKind::Burst) {
        top.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Burst,
            },
            source: None,
            target: Target::Direct(Some(character)),
        });
    }

    top.push_all_front(queue);
    DispatchResult::Continue
}
