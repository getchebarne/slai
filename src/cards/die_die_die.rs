use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DIE_DIE_DIE: Entity = make_entity_card(
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
pub static DIE_DIE_DIE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = DIE_DIE_DIE.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 17,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..DIE_DIE_DIE
};
