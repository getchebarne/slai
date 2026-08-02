use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup: upgrade 1 random Card, +5 max HP (healed), 50 gold, 1 random Potion
// See:
//    - `process_effect_relic_adopt.rs`
pub static TINY_HOUSE: Entity = make_entity_relic(RelicName::TinyHouse, RelicTier::Boss, 0, &[]);
