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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
