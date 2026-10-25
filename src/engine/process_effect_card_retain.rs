use crate::game::GameState;

// Each selection moves to the end of the hand, so the kept Cards lead the next hand in selection order
pub fn process_effect_card_retain(id_targets: &[usize], state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_retain outside the Combat frame"
    );
    assert!(!id_targets.is_empty(), "CardRetain requires id_target");
    for &id_target in id_targets {
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
}
