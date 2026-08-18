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

pub static SHIV: CardTemplate = make_card_template(
    CardName::Shiv,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Special,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 4,
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
pub static SHIV_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = SHIV.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 6,
            lifesteal: false,
        }; // +2 damage
        effects
    },
    ..SHIV
};
