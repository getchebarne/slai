use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::types::Mode;
use crate::types::Rewards;
use crate::utils::push_entity;

// Stage `count` rolled potions on the reward screen (The Lab, The Woman in Blue)
pub fn process_effect_reward_roll_potions(state: &mut GameState, count: u8) {
    // Reward memory dies with its mode; nothing may be staged here
    assert!(
        !matches!(state.mode, Mode::Reward(_)),
        "RewardRollPotions with rewards already staged"
    );

    // Roll
    let mut id_potions: Vec<usize> = Vec::new();
    for _ in 0..count {
        let potion_name = get_random_potion_name(&mut state.rng, false);
        let id = push_entity(&mut state.entities, get_potion(potion_name));
        id_potions.push(id);
    }

    state.mode = Mode::Reward(Rewards {
        id_cards: Vec::new(),
        id_relic: None,
        id_potions,
        gold: None,
    });
}
