use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
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
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 7 },
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
        let mut a = BITE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 8 }; // +1 damage
        a[1].kind = EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(3),
        }; // +1 heal
        a
    },
    ..BITE
};
