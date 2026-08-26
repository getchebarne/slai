use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Card rewards roll Rares three times as often
// See:
//    - `utils.rs::roll_card_rewards`
pub static NLOTHS_GIFT: RelicTemplate = RelicTemplate {
    name: RelicName::NlothsGift,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
