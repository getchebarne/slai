use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

// Never drinkable; consumed by the death hook in `process_effect_death`
pub static POTION_FAIRY: Entity = make_entity_potion(
    PotionName::FairyPotion,
    PotionRarity::Rare,
    false,
    true,
    &[],
);
