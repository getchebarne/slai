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

pub static SADISTIC_NATURE: CardTemplate = make_card_template(
    CardName::SadisticNature,
    CardKind::Power,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::SadisticNature,
            stacks: 5,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SADISTIC_NATURE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = SADISTIC_NATURE.effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::SadisticNature,
            stacks: 7, // +2 stacks
        };
        effects
    },
    ..SADISTIC_NATURE
};
