use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SWIFT_STRIKE: CardTemplate = make_card_template(
    CardName::SwiftStrike,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 7,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SWIFT_STRIKE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = SWIFT_STRIKE.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 10,
            lifesteal: false,
        }; // +3 damage
        effects
    },
    ..SWIFT_STRIKE
};
