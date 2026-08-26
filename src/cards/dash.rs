use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DASH: CardTemplate = make_card_template(
    CardName::Dash,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 10 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 10,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DASH_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DASH.effects;
        effects[0].kind = EffectKind::BlockGain { amount: 13 }; // +3 block
        effects[1].kind = EffectKind::DamagePhysical {
            amount: 13,
            lifesteal: false,
        }; // +3 damage
        effects
    },
    ..DASH
};
