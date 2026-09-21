use crate::consts::SHOP_PURGE_COST_INCREMENT;
use crate::game::GameState;
use crate::types::Focus;
use crate::utils::context_focus;

// The chain paid and picked; this closes the service for the visit
pub fn process_effect_shop_purge(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::Shop,
        "ShopPurge outside the Shop context"
    );

    // Ramps for the rest of the run; the next shop build reads the new value
    state.shop_purge_cost_run += SHOP_PURGE_COST_INCREMENT;

    // A shop's Card removal can be used once per visit
    state.shop.purged = true;
}
