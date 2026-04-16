use rand::Rng;

use crate::consts::MAX_MONSTERS;
use crate::state::GameState;

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

// Fills `buf` with the ids of monsters that are alive, returns how many.
// Callers use `&buf[..n]` as a slice. Zero heap allocation.
pub fn fill_alive_monster_ids(state: &GameState, buf: &mut [usize; MAX_MONSTERS]) -> usize {
    let mut n = 0;
    for i in 0..state.monster_count as usize {
        let id = state.monsters[i];
        if !state.entities[id].dead {
            buf[n] = id;
            n += 1;
        }
    }
    n
}

pub fn remove_card_from_hand(id_card: usize, hand: &mut Vec<usize>) {
    let hand_idx = hand
        .iter()
        .position(|&elem| elem == id_card)
        .expect("Can't discard a card that's not in the hand");

    hand.remove(hand_idx);
}
