use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_end.rs` (+1 max HP after each combat)
pub static FACE_OF_CLERIC: RelicTemplate = RelicTemplate {
    name: RelicName::FaceOfCleric,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
