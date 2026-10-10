use crate::game::GameState;

// Each pick moves to the end of the hand, so the kept Cards lead the next hand in pick order
pub fn process_effect_card_retain(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_retain outside the Combat frame"
    );
    let id_target = id_target.expect("CardRetain requires id_target");
    let id_card_pile_hand = &mut state.combat.id_card_pile_hand;
    if let Some(pos) = id_card_pile_hand.iter().position(|&id| id == id_target) {
        id_card_pile_hand.remove(pos);
        id_card_pile_hand.push(id_target);
    }
    let card = &mut state.entities[id_target];
    if !card.card_ethereal {
        card.card_retain = true;
    }
}
