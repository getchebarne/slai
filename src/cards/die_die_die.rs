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

pub static DIE_DIE_DIE: CardTemplate = make_card_template(
    CardName::DieDieDie,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 13,
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
pub static DIE_DIE_DIE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DIE_DIE_DIE.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 17,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..DIE_DIE_DIE
};
