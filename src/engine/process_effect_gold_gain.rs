use crate::consts::MAX_GOLD;
use crate::entity::Entity;

pub fn process_effect_gold_gain(character: &mut Entity, amount: u16) {
    character.character_gold = character
        .character_gold
        .saturating_add(amount)
        .min(MAX_GOLD);
}
