use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::CostScope;

pub fn process_effect_set_cost_override(
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u8,
    only_reduce: bool,
    scope: CostScope,
) {
    let id_target = id_target.expect("SetCostOverride requires id_target");
    let card = &mut state.entities[id_target];

    // `only_reduce` guards against cost-increase (see Enlightment)
    if only_reduce {
        if matches!(card.card_cost_kind, CardCostKind::XCost { .. }) {
            return;
        }
        let current = card.card_cost_override.map_or(card.card_cost, |o| o.amount);
        if current <= amount {
            return;
        }
    }
    match scope {
        CostScope::Combat => {
            card.card_cost = amount;
            card.card_cost_override = None;
        }
        scope => card.card_cost_override = Some(CostOverride { amount, scope }),
    }
}
