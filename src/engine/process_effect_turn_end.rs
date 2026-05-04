use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::{DispatchResult, EffectBuf};
use crate::entity::{Entity, EntityKind};
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::types::{CardName, Vitals};

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

    // Ritual: skip if newly applied
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

    // Metallicize
    // TODO: Character can trigger this too
    if modifier_has(modifiers, ModifierKind::Metallicize) {
        let stacks = modifier_stacks(modifiers, ModifierKind::Metallicize);
        queue.push_front(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_actor),
            target: Target::Direct(Some(id_actor)),
        });
    }
    DispatchResult::Continue
}

pub fn process_effect_turn_end_character(
    id_character: usize,
    entities: &mut [Entity],
    id_hand: &[usize],
    _card_target: Option<usize>,
    id_alive_monsters: &[usize],
    this_turn_discards: &mut u8,
    this_turn_attacks_played: &mut u8,
    _rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Reset per-turn counters synchronously, before the rest of the chain queues up
    *this_turn_discards = 0;
    *this_turn_attacks_played = 0;

    // Clear per-instance cost overrides (Bullet Time)
    for entity in entities.iter_mut() {
        if matches!(entity.kind, EntityKind::Card) {
            entity.card_cost_override = None;
        }
    }

    let mut buf_effects = EffectBuf::new();

    let mods_char = &entities[id_character].modifiers;
    if modifier_has(mods_char, ModifierKind::Retain) && !id_hand.is_empty() {
        let stacks = modifier_stacks(mods_char, ModifierKind::Retain);
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

    if modifier_has(mods_char, ModifierKind::Ritual)
        && !mods_char.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(mods_char, ModifierKind::Ritual);
        buf_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // WraithForm: each stack costs 1 Dexterity per player turn end
    if modifier_has(mods_char, ModifierKind::WraithForm) {
        let stacks = modifier_stacks(mods_char, ModifierKind::WraithForm);
        buf_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Dexterity,
                stacks: -stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Burn end-of-turn damage TODO: improve data representation
    for &id_card in id_hand {
        let card = &entities[id_card];
        if card.card_name == CardName::Burn {
            let dmg: u16 = if card.card_upgraded { 4 } else { 2 };
            buf_effects.push(Effect {
                kind: EffectKind::DamageDeal { amount: dmg },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
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
            // Thievery
            let stacks_thievery = if modifier_has(&monster.modifiers, ModifierKind::Thievery) {
                Some(modifier_stacks(&monster.modifiers, ModifierKind::Thievery) as u8)
            } else {
                None
            };
            for e in monster.moves[move_idx].effects.iter() {
                buf_effects.push(Effect {
                    id_source: Some(id_monster),
                    ..*e
                });
                if let Some(amount) = stacks_thievery
                    && matches!(e.kind, EffectKind::DamagePhysical { .. })
                {
                    buf_effects.push(Effect {
                        kind: EffectKind::GoldSteal { amount },
                        id_source: Some(id_monster),
                        target: Target::Direct(Some(id_character)),
                    });
                }
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

    if modifier_has(mods_char, ModifierKind::Burst) {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Burst,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    if modifier_has(mods_char, ModifierKind::NoDraw) {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::NoDraw,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Entangled removes at the end of turn
    if modifier_has(mods_char, ModifierKind::Entangled) {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Entangled,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    buf_effects.push_all_front(queue);
    DispatchResult::Continue
}
