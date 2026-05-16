use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_gold_steal(
    id_source: usize,
    id_character: usize,
    amount: u8,
    entities: &mut [Entity],
) -> Option<Phase> {
    let take = (amount as u16).min(entities[id_character].character_gold);
    if take == 0 {
        return None;
    }
    entities[id_character].character_gold -= take;
    entities[id_source].monster_stolen_gold =
        entities[id_source].monster_stolen_gold.saturating_add(take);
    None
}
