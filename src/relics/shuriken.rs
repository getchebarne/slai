use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_card_play.rs`
pub static SHURIKEN: Entity = make_entity_relic(RelicName::Shuriken, RelicTier::Uncommon, 0, &[]);
