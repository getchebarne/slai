use crate::game::GameState;
use crate::potions::find_free_slot;

pub fn process_effect_potion_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("PotionAdopt requires id_target");
    let slot = find_free_slot(&state.id_potions, state.potion_slots_max)
        .expect("PotionAdopt queuers check for a free slot");
    state.id_potions[slot] = Some(id_potion);
}
