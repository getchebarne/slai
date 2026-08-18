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

pub static SKEWER: CardTemplate = make_card_template(
    CardName::Skewer,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::XCost { offset: 0 },
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
pub static SKEWER_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = SKEWER.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 10,
            lifesteal: false,
        }; // +3 damage
        effects
    },
    ..SKEWER
};
