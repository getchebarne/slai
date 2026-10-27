use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;
use crate::types::Combat;

pub fn process_effect_card_remove(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::CardRemove, id_source, id_targets);
    assert!(
        state.combat.active,
        "process_effect_card_remove outside the Combat frame"
    );
    let Combat {
        id_card_pile_hand, ..
    } = &mut state.combat;
    let id_card = id_target.expect("CardRemove requires id_target");
    if let Some(pos) = id_card_pile_hand.iter().position(|&id| id == id_card) {
        id_card_pile_hand.remove(pos);
    }
}
