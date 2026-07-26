use rand::Rng;

use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::game::GameState;

// Snecko Oil: random 0-3 cost, permanent for the combat; skips X-cost and unplayables
pub fn process_effect_card_cost_randomize(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardCostRandomize requires id_target");
    let card = &state.entities[id_target];
    if matches!(card.card_cost_kind, CardCostKind::XCost { .. })
        || card.card_play_restriction == PlayRestriction::Never
    {
        return;
    }
    let roll = state.rng.random_range(0..=3u8);
    // Same-cost roll leaves any live per-turn override in place (StS parity)
    if roll != state.entities[id_target].card_cost {
        let card = &mut state.entities[id_target];
        card.card_cost = roll;
        card.card_cost_override = None;
    }
}
