use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SUCKER_PUNCH: Entity = make_entity_card(
    CardName::SuckerPunch,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
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
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
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
pub static SUCKER_PUNCH_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SUCKER_PUNCH.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 9 }; // +2 damage
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 2, // +1 weak
        };
        a
    },
    ..SUCKER_PUNCH
};
