use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The hand is kept at end of turn; ethereal still exhausts, retain flags still clear
// See:
//    - `process_effect_turn_end.rs`
pub static RUNIC_PYRAMID: Entity =
    make_entity_relic(RelicName::RunicPyramid, RelicTier::Boss, 0, &[]);
