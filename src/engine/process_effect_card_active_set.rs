use crate::engine::ProcessEffectResult;
use crate::state::GameState;

pub fn process_effect_card_active_set(
    card_idx: usize,
    game_state: &mut GameState,
) -> ProcessEffectResult {
    game_state.card_active = Some(card_idx);

    ProcessEffectResult::Pass
}
