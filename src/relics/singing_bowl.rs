use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// A Card reward may be skipped for +2 max HP (no heal)
// See:
//    - `action.rs`
//    - `process_effect_singing_bowl_proc.rs`
pub static SINGING_BOWL: Entity =
    make_entity_relic(RelicName::SingingBowl, RelicTier::Uncommon, 0, &[]);
