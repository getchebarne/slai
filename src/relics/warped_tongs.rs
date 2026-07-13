use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_turn_start.rs` (queues WarpedTongsProc post-draw)
//    - `process_effect_warped_tongs_proc.rs`
pub static WARPED_TONGS: Entity =
    make_entity_relic(RelicName::WarpedTongs, RelicTier::Special, 0, &[]);
