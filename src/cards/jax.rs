use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Amount;
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
use crate::types::DeltaSign;

pub static JAX: CardTemplate = make_card_template(
    CardName::Jax,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Special,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(3),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static JAX_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = JAX.effects;
        effects[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 3,
        }; // +1 strength
        effects
    },
    ..JAX
};
