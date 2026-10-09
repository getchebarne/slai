use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; no more than 6 Cards can be played per turn; the counter holds the turn's plays
// See:
//    - `process_effect_combat_start.rs`
//    - `process_card_play.rs`
//    - `utils.rs` (`play_cap_reached`)
pub static VELVET_CHOKER: RelicTemplate = RelicTemplate {
    name: RelicName::VelvetChoker,
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
