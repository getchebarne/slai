use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, choose 3 cards to transform; the results are upgraded
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_astrolabe_transform.rs`
pub static ASTROLABE: Entity = make_entity_relic(RelicName::Astrolabe, RelicTier::Boss, 0, &[]);
