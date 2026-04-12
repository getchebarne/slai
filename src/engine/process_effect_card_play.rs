use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::state::Entity;
use crate::types::CardKind;
use crate::types::EntityId;

pub fn process_effect_card_play(
    id_card: EntityId,
    _id_card_target: Option<EntityId>,
    character: EntityId,
    entities: &[Entity],
    _hand: &[EntityId],
    alive_monsters: &[EntityId],
    _rng: &mut impl Rng,
) -> ProcessEffectResult {
    let card = entities[id_card.0 as usize].kind.card_ref();

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
    let (_, char_modifiers) = entities[character.0 as usize].kind.combatant_ref();

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
            let (_, monster_modifiers) = entities[id_monster.0 as usize].kind.combatant_ref();
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
