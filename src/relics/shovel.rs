use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Dig at rest sites: gain a random Relic (granted directly, not staged as a reward)
// See:
//    - `action.rs`
pub static SHOVEL: Entity = make_entity_relic(RelicName::Shovel, RelicTier::Rare, 0, &[]);
