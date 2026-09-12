use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Incoming attacks leaving a 2-5 HP remainder deal 1 instead
// See:
//    - `process_effect_damage_deal.rs`
pub static TORII: RelicTemplate = RelicTemplate {
    name: RelicName::Torii,
    tier: RelicTier::Rare,
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
