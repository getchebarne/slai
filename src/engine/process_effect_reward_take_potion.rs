use crate::entity::Entity;
use crate::potions::find_free_slot;
use crate::types::RewardState;

pub fn process_effect_reward_take_potion(
    reward: &mut RewardState,
    entities: &mut [Entity],
    id_character: usize,
) {
    if let Some(id) = reward.id_potion.take() {
        let character = &mut entities[id_character];
        let slot = find_free_slot(&character.potion_slots, character.potion_slots_max)
            .expect("RewardTakePotion: belt full (action handler should have rejected)");
        character.potion_slots[slot] = Some(id);
    }
}
