use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Unknown rooms can no longer be monster fights
// See:
//    - `process_effect_room_enter.rs`
pub static JUZU_BRACELET: Entity =
    make_entity_relic(RelicName::JuzuBracelet, RelicTier::Common, 0, &[],
    "Regular enemy combats are no longer encountered in ? rooms.",
);
