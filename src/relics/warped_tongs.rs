use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_turn_start.rs` (queues a random hand upgrade post-draw)
pub static WARPED_TONGS: Entity =
    make_entity_relic(RelicName::WarpedTongs, RelicTier::Special, 0, &[]);
