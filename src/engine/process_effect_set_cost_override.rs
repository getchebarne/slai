use rand::Rng;

use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::types::CostScope;
use crate::utils::get_card_cost_this_turn;

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

    // X-cost and unplayable Cards keep their cost
    if matches!(card.card_cost_kind, CardCostKind::XCost { .. })
        || card.card_play_restriction == PlayRestriction::Never
    {
        return;
    }

    // Snecko Oil: roll 0..=amount instead
    let amount = if random {
        let roll = state.rng.random_range(0..=amount);

        // Same-cost roll leaves any live per-turn override in place
        if roll == state.entities[id_target].card_cost {
            return;
        }
        roll
    } else {
        amount
    };

    // Get mutable Card reference
    let card = &mut state.entities[id_target];

    // Check for `only_reduce`
    if only_reduce {
        let current = match scope {
            CostScope::Combat => card.card_cost,
            CostScope::Turn => get_card_cost_this_turn(card),
        };
        if current <= amount {
            return;
        }
    }

    match scope {
        CostScope::Combat => {
            card.card_cost = amount;
            if !only_reduce {
                card.card_cost_override = None;
            }
        }
        // A cost this turn equal to the combat cost leaves no override
        scope => {
            card.card_cost_override =
                (amount != card.card_cost).then_some(CostOverride { amount, scope })
        }
    }
}
