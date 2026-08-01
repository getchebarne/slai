use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// 3 charges: move to any room in the next row, ignoring paths
// See:
//    - `action.rs`
//    - `process_effect_room_select.rs`
pub static WING_BOOTS: Entity = make_entity_relic(RelicName::WingBoots, RelicTier::Rare, 3, &[]);
