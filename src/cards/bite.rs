use crate::cards::make_entity_card;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::DeltaSign;

pub static BITE: Entity = make_entity_card(
    CardName::Bite,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Special,
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
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(2),
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
pub static BITE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = BITE.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 8,
            lifesteal: false,
        }; // +1 damage
        effects[1].kind = EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(3),
        }; // +1 heal
        effects
    },
    ..BITE
};
