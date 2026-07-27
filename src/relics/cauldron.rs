use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 5 random potions (overflow beyond belt space is lost)
// See:
//    - `process_effect_relic_adopt.rs`
pub static CAULDRON: Entity = make_entity_relic(RelicName::Cauldron, RelicTier::Shop, 0, &[]);
