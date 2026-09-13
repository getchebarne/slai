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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
