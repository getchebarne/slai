use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::types::Mode;
use crate::utils::mode_replace;
use crate::utils::mode_top;
use crate::utils::push_entity;

// Stage `count` rolled Potions on the reward screen (The Lab, The Woman in Blue)
pub fn process_effect_reward_roll_potions(state: &mut GameState, count: u8) {
    // Reward memory dies with its mode; nothing may be staged here
    assert!(
        !matches!(mode_top(&state.mode_stack), Mode::Reward { .. }),
        "RewardRollPotions with rewards already staged"
    );

    // Roll
    let mut id_potions: Vec<usize> = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let potion_name = get_random_potion_name(&mut state.rng, false);
        let id = push_entity(&mut state.entities, get_potion(potion_name));
        id_potions.push(id);
    }

    mode_replace(
        &mut state.mode_stack,
        Mode::Reward {
            reward_id_cards: Vec::new(),
            reward_id_relics: Vec::new(),
            reward_id_potions: id_potions,
            reward_gold: None,
        },
    );
}
