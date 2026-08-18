use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

// Never drinkable; consumed by the death hook in `process_effect_death`
pub static POTION_FAIRY: PotionTemplate = PotionTemplate {
    name: PotionName::FairyPotion,
    rarity: PotionRarity::Rare,
    combat_only: true,
    effects: &[],
};
