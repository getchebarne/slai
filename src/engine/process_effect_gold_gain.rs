use crate::engine::DispatchResult;
use crate::entity::Entity;

// Add `amount` to the character's gold (saturating).
pub fn process_effect_gold_gain(
    id_character: usize,
    amount: u16,
    entities: &mut [Entity],
) -> DispatchResult {
    entities[id_character].gold = entities[id_character].gold.saturating_add(amount);
    DispatchResult::Continue
}
