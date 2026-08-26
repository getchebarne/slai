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
    effects_combat_start: &[],
};
