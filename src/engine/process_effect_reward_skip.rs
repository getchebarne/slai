use crate::game::GameState;
use crate::types::Screen;
use crate::utils::queue_room_select;

pub fn process_effect_reward_skip(state: &mut GameState) {
    state.reward_id_cards.clear();
    state.reward_id_relic = None;
    state.reward_id_potion = None;
    state.reward_gold = None;
    state.active = Screen::Map;
    queue_room_select(state);
}
