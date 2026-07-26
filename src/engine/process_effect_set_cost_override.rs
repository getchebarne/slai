use crate::entity::CardCostKind;
use crate::game::GameState;

pub fn process_effect_set_cost_override(
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u8,
    only_reduce: bool,
    permanent: bool,
) {
    let id_target = id_target.expect("SetCostOverride requires id_target");
    let card = &mut state.entities[id_target];
    // only_reduce (Enlightenment): skip X-cost and cards already at or below `amount`
    if only_reduce {
        if matches!(card.card_cost_kind, CardCostKind::XCost { .. }) {
            return;
        }
        let current = card.card_cost_override.unwrap_or(card.card_cost);
        if current <= amount {
            return;
        }
    }
    if permanent {
        card.card_cost = amount;
        card.card_cost_override = None;
    } else {
        card.card_cost_override = Some(amount);
    }
}
