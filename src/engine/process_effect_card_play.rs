use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::types::CardKind;
use crate::entities::{Entity, EntityKind};

pub fn process_effect_card_play(
    id_card: usize,
    _id_card_target: Option<usize>,
    character: usize,
    entities: &[Entity],
    _hand: &[usize],
    alive_monsters: &[usize],
    _rng: &mut impl Rng,
) -> ProcessEffectResult {
    let EntityKind::Card(card) = & entities[id_card].kind else { unreachable!() };

    // Create empty container for top effects
    let mut top_effects = Vec::new();

    // Push top effects. Start w/ energy loss
    top_effects.push(Effect {
        kind: EffectKind::EnergyLoss { amount: card.cost },
        source: None,
        target: Target::Direct(None),
    });

    // Exhaust vs. remove vs. discard
    if card.exhaust {
        top_effects.push(Effect {
            kind: EffectKind::CardExhaust,
            source: None,
            target: Target::Direct(Some(id_card)),
        })
    } else if card.kind == CardKind::Power {
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

    // Get character's modifiers
    let EntityKind::Character(c) = &entities[character].kind else { unreachable!() };
    let char_modifiers = &c.modifiers;

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
    if card.kind == CardKind::Attack {
        for &id_monster in alive_monsters {
            let EntityKind::Monster(m) = &entities[id_monster].kind else { unreachable!() };
            let monster_modifiers = &m.modifiers;
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

    // Copy the card's effects into the queue, stamping source with the character
    let card_effects: Vec<Effect> = card
        .effects
        .iter()
        .map(|e| Effect {
            source: Some(character),
            ..*e
        })
        .collect();

    // Modifier / Burst
    if modifier_has(char_modifiers, ModifierKind::Burst) && card.kind == CardKind::Skill {
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

    ProcessEffectResult::AddAndContinue {
        top: top_effects,
        bot: Vec::new(),
    }
}
