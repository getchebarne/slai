use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DRAMATIC_ENTRANCE: CardTemplate = make_card_template(
    CardName::DramaticEntrance,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 8,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DRAMATIC_ENTRANCE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DRAMATIC_ENTRANCE.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 12,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..DRAMATIC_ENTRANCE
};
