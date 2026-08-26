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

pub static BANE: CardTemplate = make_card_template(
    CardName::Bane,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 7,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::DamagePhysicalIfPoisoned { amount: 7 },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BANE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = BANE.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 10,
            lifesteal: false,
        }; // +3 damage
        effects[1].kind = EffectKind::DamagePhysicalIfPoisoned { amount: 10 }; // +3 damage
        effects
    },
    ..BANE
};
