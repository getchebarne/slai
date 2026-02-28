use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::engine::instantiate_templates;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::state::Entity;
use crate::types::CardKind;
use crate::types::EntityId;

pub fn process_effect_card_play(
    card_id: EntityId,
    entities: &[Entity],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
) -> ProcessEffectResult {
    let card = entities[card_id.0 as usize].kind.card_ref();

    let mut top_effects = Vec::new();

    top_effects.push(Effect::EnergyLoss { amount: card.cost });

    if card.exhaust {
        top_effects.push(Effect::CardExhaust { card_id })
    } else if card.kind == CardKind::Power {
        top_effects.push(Effect::CardRemove { card_id })
    } else {
        top_effects.push(Effect::CardDiscard { card_id })
    };

    let (_, character_modifiers) = entities[0].kind.combatant_ref();

    if modifier_has(character_modifiers, ModifierKind::AfterImage) {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::AfterImage);
        top_effects.push(Effect::BlockGain {
            target: EntityId(0),
            amount: stacks as u16,
            from_card: false,
        })
    }

    if modifier_has(character_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::ThousandCuts);
        for &mid in alive_monsters {
            top_effects.push(Effect::DamagePhysical {
                source: EntityId(0),
                target: mid,
                base: stacks as u16,
            });
        }
    }

    if card.kind == CardKind::Attack {
        for &mid in alive_monsters {
            let (_, monster_modifiers) = entities[mid.0 as usize].kind.combatant_ref();
            if modifier_has(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                top_effects.push(Effect::DamageDeal {
                    target: EntityId(0),
                    amount: stacks as u16,
                });
            }
        }
    }

    let card_effects = instantiate_templates(card.effects, EntityId(0), card_target, alive_monsters);

    if modifier_has(character_modifiers, ModifierKind::Burst) && card.kind == CardKind::Skill {
        top_effects.extend(card_effects.iter().cloned());
        top_effects.extend(card_effects);
        top_effects.push(Effect::ModifierGain {
            target: EntityId(0),
            kind: ModifierKind::Burst,
            stacks: -1,
        });
    } else {
        top_effects.extend(card_effects);
    }
    ProcessEffectResult::Continue {
        bot: Vec::new(),
        top: top_effects,
    }
}
