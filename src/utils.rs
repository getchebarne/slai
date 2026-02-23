use rand::Rng;

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

pub fn remove_card_from_hand(card_idx: usize, hand: &mut Vec<usize>) {
    let hand_idx = hand
        .iter()
        .position(|&elem| elem == card_idx)
        .expect("Can't discard a card that's not in the hand");

    hand.remove(hand_idx);
}
