use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::CostScope;

// Eviscerate's cost this turn restarts at its combat cost less this turn's discards
pub fn process_effect_card_cost_minus_discards(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_cost_minus_discards outside the Combat frame"
    );
    let id_card = id_target.expect("CardCostMinusDiscards requires id_target");
    let card = &mut state.entities[id_card];
    let cost = (card.card_cost as u16).saturating_sub(state.combat.this_turn_discards) as u8;
    card.card_cost_override = (cost != card.card_cost).then_some(CostOverride {
        amount: cost,
        scope: CostScope::Turn,
    });
}
