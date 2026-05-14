use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::potions::grant_potion;
use crate::types::PotionName;

pub fn process_effect_potion_add(
    name: PotionName,
    id_character: usize,
    entities: &mut Vec<Entity>,
) -> DispatchResult {
    grant_potion(entities, id_character, name);
    DispatchResult::Continue
}
