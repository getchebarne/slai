use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Card rewards roll Rares three times as often
// See:
//    - `utils.rs::roll_card_rewards`
pub static NLOTHS_GIFT: Entity =
    make_entity_relic(RelicName::NlothsGift, RelicTier::Special, 0, &[]);
