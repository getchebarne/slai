use crate::cards::make_entity_card;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::DeltaSign;

pub static JAX: Entity = make_entity_card(
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
pub static JAX_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = JAX.card_effects;
        effects[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 3,
        }; // +1 strength
        effects
    },
    ..JAX
};
