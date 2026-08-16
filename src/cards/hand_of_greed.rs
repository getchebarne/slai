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

pub static HAND_OF_GREED: Entity = make_entity_card(
    CardName::HandOfGreed,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 20,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::HandOfGreedProc { gold: 20 },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static HAND_OF_GREED_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = HAND_OF_GREED.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 25,
            lifesteal: false,
        }; // +5 damage
        effects[1].kind = EffectKind::HandOfGreedProc { gold: 25 }; // +5 gold
        effects
    },
    ..HAND_OF_GREED
};
