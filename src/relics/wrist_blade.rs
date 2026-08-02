use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Attacks that cost 0 deal 4 additional damage
// See:
//    - `process_effect_card_play.rs`
pub static WRIST_BLADE: Entity = make_entity_relic(RelicName::WristBlade, RelicTier::Boss, 0, &[]);
