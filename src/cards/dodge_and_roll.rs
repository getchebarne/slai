use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DODGE_AND_ROLL: CardTemplate = make_card_template(
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
pub static DODGE_AND_ROLL_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DODGE_AND_ROLL.effects;
        effects[0].kind = EffectKind::BlockGain { amount: 6 }; // +2 block
        effects[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnBlock,
            stacks: 6, // +2 next-turn block
        };
        effects
    },
    ..DODGE_AND_ROLL
};
