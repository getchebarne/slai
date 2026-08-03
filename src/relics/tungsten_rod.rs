use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every HP loss is reduced by 1
// See:
//    - `process_effect_health_delta.rs`
pub static TUNGSTEN_ROD: Entity =
    make_entity_relic(RelicName::TungstenRod, RelicTier::Rare, 0, &[]);
