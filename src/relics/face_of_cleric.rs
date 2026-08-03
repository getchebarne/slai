use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_end.rs` (+1 max HP after each combat)
pub static FACE_OF_CLERIC: Entity =
    make_entity_relic(RelicName::FaceOfCleric, RelicTier::Special, 0, &[]);
