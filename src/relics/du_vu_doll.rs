use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat starts with 1 Strength per Curse in the deck
// See:
//    - `process_effect_combat_start.rs`
pub static DU_VU_DOLL: Entity = make_entity_relic(RelicName::DuVuDoll, RelicTier::Rare, 0, &[]);
