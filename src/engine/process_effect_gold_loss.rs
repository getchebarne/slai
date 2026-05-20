use crate::entity::Entity;

pub fn process_effect_gold_loss(character: &mut Entity, amount: u16) {
    character.character_gold = character.character_gold.saturating_sub(amount);
}
