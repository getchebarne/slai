use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Shop prices are halved
// See:
//    - `process_effect_shop_build.rs`
//    - `process_effect_shop_buy_relic.rs`
pub static MEMBERSHIP_CARD: RelicTemplate = RelicTemplate {
    name: RelicName::MembershipCard,
    tier: RelicTier::Shop,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
