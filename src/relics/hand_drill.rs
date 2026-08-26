use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Breaking an enemy's block applies 2 Vulnerable
// See:
//    - `process_effect_damage_deal.rs`
pub static HAND_DRILL: RelicTemplate = RelicTemplate {
    name: RelicName::HandDrill,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
