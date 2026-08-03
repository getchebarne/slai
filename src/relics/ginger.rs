use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The character can no longer become Weakened
// See:
//    - `process_effect_modifier_gain.rs`
pub static GINGER: Entity = make_entity_relic(RelicName::Ginger, RelicTier::Rare, 0, &[]);
