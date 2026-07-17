use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::types::Screen;
use crate::utils::push_entity;

// Stage `count` rolled potions on the reward screen (The Lab, The Woman in Blue)
pub fn process_effect_reward_roll_potions(state: &mut GameState, count: u8) {
    // Reward memory is cleared on room exit; nothing may be staged here
    assert!(
        state.reward_id_potions.is_empty(),
        "RewardRollPotions with potions already staged"
    );

    // Roll
    for _ in 0..count {
        let potion_name = get_random_potion_name(&mut state.rng, false);
        let id = push_entity(&mut state.entities, get_potion(potion_name));
        state.reward_id_potions.push(id);
    }

    // Upate screen
    state.screen = Screen::Reward;
}
