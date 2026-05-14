use rand::Rng;

use crate::consts::{FACTOR_VULN, FACTOR_WEAK, MAX_MONSTERS};
use crate::game::GameState;

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

// Fills `buf` with the ids of monsters that are alive, returns how many
// Callers use `&buf[..n]` as a slice. Zero heap allocation
pub fn fill_alive_monster_ids(state: &GameState, buf_alive: &mut [usize; MAX_MONSTERS]) -> usize {
    let mut n = 0;
    for i in 0..state.monster_count as usize {
        let id_monster = state.id_monsters[i];
        if !state.entities[id_monster].dead {
            buf_alive[n] = id_monster;
            n += 1;
        }
    }
    n
}

// Strength, Weak, and Vulnerable scaling shared between the live damage pipeline
// and the FFI intent view
pub fn scale_attack_damage(
    base: u16,
    source_str_stacks: i16,
    source_is_weak: bool,
    target_is_vulnerable: bool,
) -> u16 {
    let mut value = base as f32 + source_str_stacks as f32;
    if source_is_weak {
        value *= FACTOR_WEAK;
    }
    if target_is_vulnerable {
        value *= FACTOR_VULN;
    }
    value.max(0.0) as u16
}

pub fn remove_card_from_hand(id_target: usize, id_hand: &mut Vec<usize>) {
    let hand_idx = id_hand
        .iter()
        .position(|&elem| elem == id_target)
        .expect("Can't discard a card that's not in the hand");

    id_hand.remove(hand_idx);
}
