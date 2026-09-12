use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Purchased shop stock restocks; 20% shop discount
// See:
//    - `process_effect_shop_build.rs`
//    - `process_effect_shop_buy_card.rs`
//    - `process_effect_shop_buy_relic.rs`
//    - `process_effect_shop_buy_potion.rs`
pub static THE_COURIER: RelicTemplate = RelicTemplate {
    name: RelicName::TheCourier,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
