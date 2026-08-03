use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_FIRE: Entity = make_entity_potion(
    PotionName::FirePotion,
    PotionRarity::Common,
    true,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 20 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
);
