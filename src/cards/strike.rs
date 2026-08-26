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

pub static STRIKE: CardTemplate = make_card_template(
    CardName::Strike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Basic,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 6,
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
pub static STRIKE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = STRIKE.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 9,
            lifesteal: false,
        }; // +3 damage
        effects
    },
    ..STRIKE
};
