use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Unknown Rooms can no longer be Monster fights
// See:
//    - `process_effect_room_enter.rs`
pub static JUZU_BRACELET: RelicTemplate = RelicTemplate {
    name: RelicName::JuzuBracelet,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
