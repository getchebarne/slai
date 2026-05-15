use crate::entity::Entity;
use crate::potions::grant_potion;
use crate::types::PotionName;
use crate::types::Phase;

pub fn process_effect_potion_add(
    name: PotionName,
    id_character: usize,
    entities: &mut Vec<Entity>,
) -> Option<Phase> {
    grant_potion(entities, id_character, name);
    None
}
