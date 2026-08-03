use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite combats start with 2 Strength
// See:
//    - `process_effect_combat_start.rs`
pub static SLING_OF_COURAGE: Entity =
    make_entity_relic(RelicName::SlingOfCourage, RelicTier::Shop, 0, &[]);
