use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::DeltaSign;

pub static BANDAGE_UP: CardTemplate = make_card_template(
    CardName::BandageUp,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(4),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BANDAGE_UP_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = BANDAGE_UP.effects;
        effects[0].kind = EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(6),
        }; // +2 heal
        effects
    },
    ..BANDAGE_UP
};
