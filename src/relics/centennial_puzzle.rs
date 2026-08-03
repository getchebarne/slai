use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first HP loss each combat draws 3 Cards
// See:
//    - `process_effect_health_delta.rs`
pub static CENTENNIAL_PUZZLE: Entity =
    make_entity_relic(RelicName::CentennialPuzzle, RelicTier::Common, 0, &[]);
