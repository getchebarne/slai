use crate::consts::MAX_GOLD;
use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_gold_gain(character: &mut Entity, amount: u16) -> Option<Phase> {
    character.character_gold = character
        .character_gold
        .saturating_add(amount)
        .min(MAX_GOLD);
    None
}
