use rand::Rng;

use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::types::CostScope;

pub fn process_effect_set_cost_override(
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u8,
    only_reduce: bool,
    random: bool,
    scope: CostScope,
) {
    let id_target = id_target.expect("SetCostOverride requires id_target");
    let card = &state.entities[id_target];

    // Snecko Oil: roll 0..=amount instead; X-cost and unplayables skip before rolling
    let amount = if random {
        if matches!(card.card_cost_kind, CardCostKind::XCost { .. })
            || card.card_play_restriction == PlayRestriction::Never
        {
            return;
        }
        let roll = state.rng.random_range(0..=amount);
        // Same-cost roll leaves any live per-turn override in place (StS parity)
        if roll == state.entities[id_target].card_cost {
            return;
        }
        roll
    } else {
        amount
    };

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
