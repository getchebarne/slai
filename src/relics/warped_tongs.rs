use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_turn_start.rs` (queues a random hand upgrade post-draw)
pub static WARPED_TONGS: RelicTemplate = RelicTemplate {
    name: RelicName::WarpedTongs,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
