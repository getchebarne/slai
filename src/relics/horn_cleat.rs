use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the start of turn 2 each combat, gain 14 block
// See:
//    - `process_effect_turn_start.rs`
pub static HORN_CLEAT: Entity =
    make_entity_relic(RelicName::HornCleat, RelicTier::Uncommon, 0, &[]);
