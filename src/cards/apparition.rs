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

pub static APPARITION: CardTemplate = make_card_template(
    CardName::Apparition,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Special,
    1,
    CardCostKind::Fixed,
    false,
    true,
    true,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Intangible,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static APPARITION_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    ethereal: false, // Upgrade removes Ethereal
    ..APPARITION
};
