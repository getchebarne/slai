use rand::Rng;

use crate::entity::Entity;
use crate::potions::get_random_potion;
use crate::potions::grant_potion;
use crate::types::Phase;

pub fn process_effect_potion_add_random(
    limited: bool,
    id_character: usize,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> Option<Phase> {
    let name = get_random_potion(rng, limited);
    grant_potion(entities, id_character, name);
    None
}
