use crate::game::GameState;

// The run is over; a won run queues this as a phase, so the combat-end Relics resolve first
pub fn process_effect_game_over(state: &mut GameState) {
    state.game_over = true;
}
