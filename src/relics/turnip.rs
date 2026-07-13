use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The character can no longer become Frail
// See:
//    - `process_effect_modifier_gain.rs`
pub static TURNIP: Entity = make_entity_relic(RelicName::Turnip, RelicTier::Rare, 0, &[]);
