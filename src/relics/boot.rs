use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Player attacks leaving a 1-4 HP remainder deal 5 instead
// See:
//    - `process_effect_damage_deal.rs`
pub static BOOT: RelicTemplate = RelicTemplate {
    name: RelicName::Boot,
    tier: RelicTier::Common,
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
