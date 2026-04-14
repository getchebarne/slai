use rand::Rng;

use crate::state::GameState;
use crate::entities::EntityKind;

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

pub fn get_alive_monster_ids(state: &GameState) -> Vec<usize> {
    state.monsters[..state.monster_count as usize]
        .iter()
        .copied()
        .filter(|&id| {
            let EntityKind::Monster(m) = &state.entities[id].kind else {
                unreachable!()
            };
            !m.dead
        })
        .collect()
}

pub fn remove_card_from_hand(id_card: usize, hand: &mut Vec<usize>) {
    let hand_idx = hand
        .iter()
        .position(|&elem| elem == id_card)
        .expect("Can't discard a card that's not in the hand");

    hand.remove(hand_idx);
}
