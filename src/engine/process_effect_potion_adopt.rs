use crate::game::GameState;
use crate::potions::find_free_slot;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_potion_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("PotionAdopt requires id_target");

    // Sozu: potions can no longer be obtained (destroyed, not deferred)
    if has_relic(&state.id_relics, RelicName::Sozu) {
        return;
    }

    let slot = find_free_slot(&state.id_potions, state.potion_slots_max)
        .expect("PotionAdopt queuers check for a free slot");
    state.id_potions[slot] = Some(id_potion);
}
