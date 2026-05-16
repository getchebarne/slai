use crate::entity::Entity;
use crate::potions::find_free_slot;
use crate::types::Phase;

pub fn process_effect_reward_take_potion(
    phase: &mut Phase,
    entities: &mut [Entity],
    id_character: usize,
) -> Option<Phase> {
    if let Phase::Reward { id_potion, .. } = phase {
        if let Some(id) = id_potion.take() {
            let character = &mut entities[id_character];
            let slot = find_free_slot(&character.potion_slots, character.potion_slots_max)
                .expect("RewardTakePotion: belt full (action handler should have rejected)");
            character.potion_slots[slot] = Some(id);
        }
    }
    None
}
