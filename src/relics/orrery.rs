use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On purchase, stages 5 Card bundles in the shop
// See:
//    - `process_effect_relic_adopt.rs`
pub static ORRERY: Entity = make_entity_relic(RelicName::Orrery, RelicTier::Shop, 0, &[]);
