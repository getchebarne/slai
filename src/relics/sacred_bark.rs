use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Potions made while Bark is held, or in the belt when it is picked up, double their effects for good
// See:
//    - `potions/mod.rs`
//    - `process_effect_potion_use.rs`
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_death.rs`
pub static SACRED_BARK: RelicTemplate = RelicTemplate {
    name: RelicName::SacredBark,
    tier: RelicTier::Boss,
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
