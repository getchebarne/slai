use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_EXPLOSIVE: Entity = make_entity_potion(
    PotionName::ExplosivePotion,
    PotionRarity::Common,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 10,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
);
