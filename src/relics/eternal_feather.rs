use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Entering a rest site heals 3 HP per 5 deck Cards
// See:
//    - `process_effect_room_enter.rs`
pub static ETERNAL_FEATHER: Entity =
    make_entity_relic(RelicName::EternalFeather, RelicTier::Uncommon, 0, &[]);
