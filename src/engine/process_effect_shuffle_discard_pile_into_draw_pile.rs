use crate::utils::reshuffle_discard_into_draw;
use rand::Rng;

pub fn process_effect_shuffle_discard_pile_into_draw_pile(
    id_pile_draw: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    rng: &mut impl Rng,
) {
    reshuffle_discard_into_draw(id_pile_draw, id_pile_discard, rng);
}
