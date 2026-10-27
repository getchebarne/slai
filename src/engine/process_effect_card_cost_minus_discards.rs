use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::CostScope;

// Eviscerate's cost this turn restarts at its combat cost less this turn's discards
pub fn process_effect_card_cost_minus_discards(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(
        state,
        EffectKind::CardCostMinusDiscards,
        id_source,
        id_targets,
    );
    assert!(
        state.combat.active,
        "process_effect_card_cost_minus_discards outside the Combat frame"
    );
    let id_card = id_target.expect("CardCostMinusDiscards requires id_target");
    let card = &mut state.entities[id_card];
    let cost = card
        .card_cost
        .saturating_sub(state.combat.this_turn_discards);
    card.card_cost_override = (cost != card.card_cost).then_some(CostOverride {
        amount: cost,
        scope: CostScope::Turn,
    });
}
