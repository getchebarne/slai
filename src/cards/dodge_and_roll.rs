use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DODGE_AND_ROLL: Entity = make_entity_card(
    CardName::DodgeAndRoll,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 4 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnBlock,
                stacks: 4,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DODGE_AND_ROLL_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DODGE_AND_ROLL.card_effects;
        a[0].kind = EffectKind::BlockGain { amount: 6 }; // +2 block
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnBlock,
            stacks: 6, // +2 next-turn block
        };
        a
    },
    ..DODGE_AND_ROLL
};
