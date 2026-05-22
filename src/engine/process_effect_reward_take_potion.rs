use crate::game::GameState;
use crate::potions::find_free_slot;

pub fn process_effect_reward_take_potion(state: &mut GameState) {
    if let Some(id) = state.reward_id_potion.take() {
        let character = &mut state.entities[state.id_character];
        let slot = find_free_slot(&character.potion_slots, character.potion_slots_max)
            .expect("RewardTakePotion: belt full (action handler should have rejected)");
        character.potion_slots[slot] = Some(id);
    }
}
