use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_room_enter.rs` (gain 50 gold on entering a "?" room)
pub static SSSERPENT_HEAD: Entity =
    make_entity_relic(RelicName::SsserpentHead, RelicTier::Special, 0, &[]);
