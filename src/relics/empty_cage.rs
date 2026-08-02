use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, remove 2 Cards from the deck
// See:
//    - `process_effect_relic_adopt.rs`
pub static EMPTY_CAGE: Entity = make_entity_relic(RelicName::EmptyCage, RelicTier::Boss, 0, &[]);
