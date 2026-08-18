use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Inert (flavor only); a Face Trader trade outcome
pub static CULTIST_HEADPIECE: RelicTemplate = RelicTemplate {
    name: RelicName::CultistHeadpiece,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
