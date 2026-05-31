use crate::game::GameState;
use crate::potions::remove_potion;

pub fn process_effect_potion_discard(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("PotionDiscard requires id_target");
    let id_character = state.id_character;
    remove_potion(&mut state.entities[id_character], id_potion);
}
