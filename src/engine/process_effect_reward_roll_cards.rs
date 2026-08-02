use crate::game::GameState;
use crate::types::Mode;
use crate::utils::card_reward_count;
use crate::utils::mode_replace;
use crate::utils::roll_card_rewards;

// Dream Catcher: a Card-only reward, rolled like a combat reward (Busted Crown applies)
pub fn process_effect_reward_roll_cards(state: &mut GameState) {
    let mut id_cards: Vec<usize> = Vec::new();
    roll_card_rewards(
        state.id_character,
        &mut state.entities,
        &mut state.rng,
        &mut id_cards,
        &state.id_relics,
        card_reward_count(&state.id_relics),
    );

    mode_replace(
        &mut state.mode_stack,
        Mode::Reward {
            reward_id_cards: vec![id_cards],
            reward_id_relics: Vec::new(),
            reward_id_potions: Vec::new(),
            reward_gold: None,
        },
    );
}
