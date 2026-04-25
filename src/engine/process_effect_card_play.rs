use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::{DispatchResult, EffectBuf};
use crate::entity::Entity;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::types::CardKind;

pub fn process_effect_card_play(
    id_card: usize,
    _id_card_target: Option<usize>,
    id_character: usize,
    entities: &[Entity],
    _hand: &[usize],
    alive_monsters: &[usize],
    _rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let card = &entities[id_card];

    // Stack locals
    let mut buf_effects = EffectBuf::new();

    buf_effects.push(Effect {
        kind: EffectKind::EnergyLoss {
            amount: card.card_cost,
        },
        id_source: None,
        target: Target::Direct(None),
    });

    if card.card_exhaust {
        buf_effects.push(Effect {
            kind: EffectKind::CardExhaust,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    } else if card.card_kind == CardKind::Power {
        buf_effects.push(Effect {
            kind: EffectKind::CardRemove,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    } else {
        buf_effects.push(Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    let char_modifiers = &entities[id_character].modifiers;

    // AfterImage
    if modifier_has(char_modifiers, ModifierKind::AfterImage) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::AfterImage);
        buf_effects.push(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    // ThousandCuts
    if modifier_has(char_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::ThousandCuts);
        for &id_monster in alive_monsters {
            buf_effects.push(Effect {
                kind: EffectKind::DamagePhysical {
                    amount: stacks as u16,
                },
                id_source: Some(id_character),
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // SharpHide (reflects on attack cards)
    if card.card_kind == CardKind::Attack {
        for &id_monster in alive_monsters {
            let monster_modifiers = &entities[id_monster].modifiers;
            if modifier_has(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                buf_effects.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: stacks as u16,
                    },
                    id_source: Some(id_monster),
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
    }

    // Card effects. Burst (skill-only) doubles them.
    let burst = modifier_has(char_modifiers, ModifierKind::Burst) && card.card_kind == CardKind::Skill;
    let reps = if burst { 2 } else { 1 };
    for _ in 0..reps {
        for e in card.card_effects.iter() {
            buf_effects.push(Effect {
                id_source: Some(id_character),
                ..*e
            });
        }
    }
    if burst {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Burst,
                stacks: -1,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    buf_effects.push_all_front(queue);
    DispatchResult::Continue
}
