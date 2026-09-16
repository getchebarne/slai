use rand::Rng;

use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::types::CostScope;
use crate::utils::get_card_effective_cost;

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

    // Ignore XCost cards
    if matches!(card.card_cost_kind, CardCostKind::XCost { .. })
        && matches!(scope, CostScope::Turn | CostScope::Combat)
    {
        return;
    }

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

    // `only_reduce` guards against cost-increase (see Enlightment). A per-turn cut reads the
    // live cost, a permanent one the printed cost; Java tests the two independently
    let (this_turn_discards, energy_current) = if state.combat.active {
        (
            state.combat.this_turn_discards,
            state.combat.energy.energy_current,
        )
    } else {
        (0, 0)
    };
    let card = &mut state.entities[id_target];

    if only_reduce {
        if matches!(card.card_cost_kind, CardCostKind::XCost { .. }) {
            return;
        }
        let current = match scope {
            CostScope::Combat => card.card_cost,
            _ => get_card_effective_cost(card, this_turn_discards, energy_current),
        };
        if current <= amount {
            return;
        }
    }

    match scope {
        // Madness and Confusion write costForTurn in the same breath; a guarded permanent
        // cut (Enlightenment+) lowers the printed cost alone
        CostScope::Combat => {
            card.card_cost = amount;
            if !only_reduce {
                card.card_cost_override = None;
            }
        }
        scope => card.card_cost_override = Some(CostOverride { amount, scope }),
    }
}
