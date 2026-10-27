use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;

// Each selection moves to the end of the hand, so the kept Cards lead the next hand in selection order
pub fn process_effect_card_retain(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::CardRetain, id_source, id_targets);
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
