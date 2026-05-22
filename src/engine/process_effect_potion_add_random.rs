use crate::game::GameState;
use crate::potions::get_random_potion;
use crate::potions::grant_potion;

pub fn process_effect_potion_add_random(state: &mut GameState, limited: bool) {
    let name = get_random_potion(&mut state.rng, limited);
    grant_potion(&mut state.entities, state.id_character, name);
}
