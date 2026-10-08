use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; the Monsters' upcoming moves are hidden
// See:
//    - `process_effect_combat_start.rs`
//    - `ffi/monster.rs`
pub static RUNIC_DOME: RelicTemplate = RelicTemplate {
    name: RelicName::RunicDome,
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
