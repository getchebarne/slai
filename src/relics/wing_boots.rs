use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// 3 charges: move to any room in the next row, ignoring paths
// See:
//    - `action.rs`
//    - `process_effect_room_select.rs`
pub static WING_BOOTS: RelicTemplate = RelicTemplate {
    name: RelicName::WingBoots,
    tier: RelicTier::Rare,
    counter_init: 3,
    effects_combat_start: &[],
};
