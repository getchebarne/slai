use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; a Monster gains 1 Strength as it spawns or splits, and a Darkling as it reincarnates
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_philosopher_stone_proc.rs`
pub static PHILOSOPHER_STONE: RelicTemplate = RelicTemplate {
    name: RelicName::PhilosopherStone,
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
