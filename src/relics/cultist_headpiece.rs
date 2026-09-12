use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Inert (flavor only); a Face Trader trade outcome
pub static CULTIST_HEADPIECE: RelicTemplate = RelicTemplate {
    name: RelicName::CultistHeadpiece,
    tier: RelicTier::Special,
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
