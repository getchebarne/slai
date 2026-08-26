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

pub static DOPPELGANGER: CardTemplate = make_card_template(
    CardName::Doppelganger,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::XCost { offset: 0 },
    false,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DrawCardNextTurn,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
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
pub static DOPPELGANGER_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    cost_kind: CardCostKind::XCost { offset: 1 }, // +1 offset
    ..DOPPELGANGER
};
