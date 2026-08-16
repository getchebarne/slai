use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::potions::get_random_potion_name_uniform;
use crate::types::reward_ensure;
use crate::utils::push_entity;

// Stage `count` rolled Potions on the Reward context (The Lab, The Woman in Blue)
pub fn process_effect_reward_roll_potions(state: &mut GameState, count: u8, uniform: bool) {
    let mut id_rolled: Vec<usize> = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let potion_name = if uniform {
            get_random_potion_name_uniform(&mut state.rng)
        } else {
            get_random_potion_name(&mut state.rng, false)
        };
        id_rolled.push(push_entity(&mut state.entities, get_potion(potion_name)));
    }

    reward_ensure(&mut state.reward);
    state.reward.id_potions.extend(id_rolled);
}
