use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Shop prices are halved
// See:
//    - `process_effect_shop_build.rs`
//    - `process_effect_shop_buy_relic.rs`
pub static MEMBERSHIP_CARD: Entity =
    make_entity_relic(RelicName::MembershipCard, RelicTier::Shop, 0, &[]);
