use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Boss combats start with a 25 HP heal
// See:
//    - `process_effect_room_enter.rs`
pub static PANTOGRAPH: Entity =
    make_entity_relic(RelicName::Pantograph, RelicTier::Uncommon, 0, &[]);
