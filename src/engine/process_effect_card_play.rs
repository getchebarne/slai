use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::types::CardKind;
use crate::entity::Entity;

pub fn process_effect_card_play(
    id_card: usize,
    _id_card_target: Option<usize>,
    character: usize,
    entities: &[Entity],
    _hand: &[usize],
    alive_monsters: &[usize],
    _rng: &mut impl Rng,
) -> ProcessEffectResult {
    let card = &entities[id_card];

    let mut top_effects = Vec::new();

    top_effects.push(Effect {
        kind: EffectKind::EnergyLoss {
            amount: card.card_cost,
        },
        source: None,
        target: Target::Direct(None),
    });

    if card.card_exhaust {
        top_effects.push(Effect {
            kind: EffectKind::CardExhaust,
            source: None,
            target: Target::Direct(Some(id_card)),
        })
    } else if card.card_kind == CardKind::Power {
        top_effects.push(Effect {
            kind: EffectKind::CardRemove,
            source: None,
            target: Target::Direct(Some(id_card)),
        })
    } else {
        top_effects.push(Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Direct(Some(id_card)),
        })
    };

    let char_modifiers = &entities[character].modifiers;

    // Modifier / After Image
    if modifier_has(char_modifiers, ModifierKind::AfterImage) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::AfterImage);
        top_effects.push(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            source: Some(character),
            target: Target::Direct(Some(character)),
        })
    }

    // Modifier / Thousand Cuts
    if modifier_has(char_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::ThousandCuts);
        for &id_monster in alive_monsters {
            top_effects.push(Effect {
                kind: EffectKind::DamagePhysical {
                    base: stacks as u16,
                },
                source: Some(character),
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // Modifier / Sharp Hide
    if card.card_kind == CardKind::Attack {
        for &id_monster in alive_monsters {
            let monster_modifiers = &entities[id_monster].modifiers;
            if modifier_has(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                top_effects.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: stacks as u16,
                    },
                    source: Some(id_monster),
                    target: Target::Direct(Some(character)),
                });
            }
        }
    }

    let card_effects: Vec<Effect> = card
        .card_effects
        .iter()
        .map(|e| Effect {
            source: Some(character),
            ..*e
        })
        .collect();

    if modifier_has(char_modifiers, ModifierKind::Burst) && card.card_kind == CardKind::Skill {
        top_effects.extend(card_effects.iter().cloned());
        top_effects.extend(card_effects);
        top_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Burst,
                stacks: -1,
            },
            source: Some(character),
            target: Target::Direct(Some(character)),
        });
    } else {
        top_effects.extend(card_effects);
    }

    ProcessEffectResult::Continue {
        top: top_effects,
        bot: Vec::new(),
    }
}
