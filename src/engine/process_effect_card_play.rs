use crate::cards::Card;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::engine::instantiate_templates;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::monsters::Monster;
use crate::state::Character;
use crate::types::EntityId;
use crate::types::CardKind;

pub fn process_effect_card_play(
    card_idx: usize,
    character: &Character,
    monsters: &Vec<Monster>,
    card_target: Option<EntityId>,
    combat_cards: &Vec<Card>,
    character_id: EntityId,
) -> ProcessEffectResult {
    let card = combat_cards[card_idx];

    let mut top_effects = Vec::new();

    top_effects.push(Effect::EnergyLoss { amount: card.cost });

    if card.exhaust {
        top_effects.push(Effect::CardExhaust { card_idx })
    } else if card.kind == CardKind::Power {
        top_effects.push(Effect::CardRemove { card_idx })
    } else {
        top_effects.push(Effect::CardDiscard { card_idx })
    };

    let character_modifiers = &character.vitals.modifiers;
    if modifier_has(character_modifiers, ModifierKind::AfterImage) {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::AfterImage);
        top_effects.push(Effect::BlockGain {
            target: character_id,
            amount: stacks as u16,
            from_card: false,
        })
    }

    if modifier_has(character_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::ThousandCuts);
        for monster in monsters.iter() {
            top_effects.push(Effect::DamagePhysical {
                source: character_id,
                target: monster.id,
                base: stacks as u16,
            })
        }
    }

    if card.kind == CardKind::Attack {
        for monster in monsters.iter() {
            let monster_modifiers = &monster.vitals.modifiers;
            if modifier_has(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                top_effects.push(Effect::DamageDeal {
                    target: character_id,
                    amount: stacks as u16,
                })
            }
        }
    }

    let card_effects =
        instantiate_templates(&card.effects, character_id, card_target, character_id, monsters);

    if modifier_has(&character_modifiers, ModifierKind::Burst) && card.kind == CardKind::Skill {
        top_effects.extend(card_effects.iter().cloned());
        top_effects.extend(card_effects);
        top_effects.push(Effect::ModifierGain {
            target: character_id,
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
