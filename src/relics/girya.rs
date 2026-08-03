use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Lift at rest sites (max 3); combats open with Strength equal to lifts
// See:
//    - `action.rs`
//    - `process_effect_girya_lift.rs`
//    - `process_effect_combat_start.rs`
pub static GIRYA: Entity = make_entity_relic(RelicName::Girya, RelicTier::Rare, 0, &[]);
