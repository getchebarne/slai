use crate::game::GameState;
use crate::utils::reshuffle_discard_into_draw;

pub fn process_effect_shuffle_discard_pile_into_draw_pile(state: &mut GameState) {
    reshuffle_discard_into_draw(
        &mut state.id_pile_draw,
        &mut state.id_pile_discard,
        &mut state.rng,
    );
}
