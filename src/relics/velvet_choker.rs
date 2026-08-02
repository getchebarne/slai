use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; no more than 6 Cards can be played per turn
// See:
//    - `process_effect_combat_start.rs`
//    - `action.rs`
pub static VELVET_CHOKER: Entity =
    make_entity_relic(RelicName::VelvetChoker, RelicTier::Boss, 0, &[]);
