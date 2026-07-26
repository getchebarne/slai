use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

// Never drinkable; consumed by the death hook in `process_effect_death`
pub static FAIRY_POTION: Entity = make_entity_potion(
    PotionName::FairyPotion,
    PotionRarity::Rare,
    false,
    true,
    &[],
);
