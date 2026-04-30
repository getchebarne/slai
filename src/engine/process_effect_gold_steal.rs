use crate::engine::DispatchResult;
use crate::entity::Entity;

// Looter Mug/Lunge: transfer min(amount, character.gold) from the character
// to the source monster's stolen_gold pile. Cap source += transfer.
pub fn process_effect_gold_steal(
    id_source: usize,
    id_character: usize,
    amount: u8,
    entities: &mut [Entity],
) -> DispatchResult {
    let take = (amount as u16).min(entities[id_character].gold);
    if take == 0 {
        return DispatchResult::Continue;
    }
    entities[id_character].gold -= take;
    entities[id_source].stolen_gold = entities[id_source].stolen_gold.saturating_add(take);
    DispatchResult::Continue
}
