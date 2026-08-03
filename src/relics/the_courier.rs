use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Purchased shop stock restocks; 20% shop discount
// See:
//    - `process_effect_shop_build.rs`
//    - `process_effect_shop_buy_card.rs`
//    - `process_effect_shop_buy_relic.rs`
//    - `process_effect_shop_buy_potion.rs`
pub static THE_COURIER: Entity =
    make_entity_relic(RelicName::TheCourier, RelicTier::Uncommon, 0, &[]);
