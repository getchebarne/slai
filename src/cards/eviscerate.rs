use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static EVISCERATE: Entity = make_entity_card(
    CardName::Eviscerate,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    3,
    CardCostKind::MinusDiscardsThisTurn,
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
            kind: EffectKind::DamagePhysical {
                amount: 7,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 7,
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
pub static EVISCERATE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = EVISCERATE.card_effects;
        let upgraded_kind = EffectKind::DamagePhysical {
            amount: 9,
            lifesteal: false,
        }; // +2 damage
        effects[0].kind = upgraded_kind;
        effects[1].kind = upgraded_kind;
        effects[2].kind = upgraded_kind;
        effects
    },
    ..EVISCERATE
};
