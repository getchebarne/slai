use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th Attack deals double damage; counter persists across combats
// See:
//    - `process_effect_card_play.rs`
//    - `process_effect_combat_start.rs`
pub static PEN_NIB: Entity = make_entity_relic(RelicName::PenNib, RelicTier::Common, 0, &[]);
