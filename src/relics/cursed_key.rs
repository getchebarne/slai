use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; opening a chest adds a random Curse to the deck
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_chest_open.rs`
pub static CURSED_KEY: Entity = make_entity_relic(RelicName::CursedKey, RelicTier::Boss, 0, &[]);
