use crate::cards::Card;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::engine::instantiate_templates;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::monsters::Monster;
use crate::state::Character;
use crate::types::ActorId;
use crate::types::CardKind;

pub fn process_effect_card_play(
    card_idx: usize,
    character: &Character,
    monsters: &Vec<Monster>,
    card_target: Option<u8>,
    combat_cards: &Vec<Card>,
) -> ProcessEffectResult {
    let card = combat_cards[card_idx];

    // Initialize empty top-effects
    let mut top_effects = Vec::new();

    // Energy loss
    top_effects.push(Effect::EnergyLoss { amount: card.cost });

    // Exhaust / power / discard
    if card.exhaust {
        top_effects.push(Effect::CardExhaust { card_idx })
    } else if card.kind == CardKind::Power {
        top_effects.push(Effect::CardRemove { card_idx })
    } else {
        top_effects.push(Effect::CardDiscard { card_idx })
    };

    // Modifier triggers
    // After Image
    let character_modifiers = &character.vitals.modifiers;
    if modifier_has(character_modifiers, ModifierKind::AfterImage) {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::AfterImage);
        top_effects.push(Effect::BlockGain {
            target: ActorId::Character,
            amount: stacks as u16,
            from_card: false,
        })
    }

    // Thousand Cuts
    if modifier_has(character_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::ThousandCuts);
        for i in 0..monsters.len() {
            top_effects.push(Effect::DamagePhysical {
                source: ActorId::Character,
                target: ActorId::Monster(i as u8),
                base: stacks as u16,
            })
        }
    }

    // Sharp hide
    if card.kind == CardKind::Attack {
        for (i, monster) in monsters.iter().enumerate() {
            let monster_modifiers = &monster.vitals.modifiers;
            if modifier_has(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                top_effects.push(Effect::DamageDeal {
                    target: ActorId::Character,
                    amount: stacks as u16,
                })
            }
        }
    }

    // Card's effects
    let card_effects =
        instantiate_templates(&card.effects, ActorId::Character, card_target, monsters);

    if modifier_has(&character_modifiers, ModifierKind::Burst) && card.kind == CardKind::Skill {
        top_effects.extend(card_effects.iter().cloned());
        top_effects.extend(card_effects);
        top_effects.push(Effect::ModifierGain {
            target: ActorId::Character,
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
