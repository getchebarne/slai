use crate::engine::ProcessEffectResult;
use crate::state::GameState;

pub fn process_effect_card_active_clear(game_state: &mut GameState) -> ProcessEffectResult {
    game_state.card_active = None;

    ProcessEffectResult::Pass
}
