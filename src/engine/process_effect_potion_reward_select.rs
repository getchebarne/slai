use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::engine::try_complete_reward;
use crate::entity::Entity;
use crate::potions::find_free_slot;
use crate::types::Phase;

pub fn process_effect_potion_reward_select(
    phase: &mut Phase,
    entities: &mut [Entity],
    id_character: usize,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if let Phase::Reward { id_potion, .. } = phase {
        if let Some(id) = id_potion.take() {
            let character = &mut entities[id_character];
            let slot = find_free_slot(&character.potion_slots, character.potion_slots_max)
                .expect("PotionRewardSelect: belt full (action handler should have rejected)");
            character.potion_slots[slot] = Some(id);
        }
    }
    try_complete_reward(phase, effect_queue);
    DispatchResult::Continue
}
