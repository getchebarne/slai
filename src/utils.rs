pub fn remove_card_from_hand(card_idx: usize, hand: &mut Vec<usize>) {
    let hand_idx = hand
        .iter()
        .position(|&elem| elem == card_idx)
        .expect("Can't discard a card that's not in the hand");

    hand.remove(hand_idx);
}
