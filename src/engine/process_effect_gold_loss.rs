use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_gold_loss(character: &mut Entity, amount: u16) -> Option<Phase> {
    character.character_gold = character.character_gold.saturating_sub(amount);
    None
}
