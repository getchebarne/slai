use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Entering any room grants 12 gold until gold is spent at a shop
// See:
//    - `process_effect_gold_delta.rs`
//    - `process_effect_room_enter.rs`
pub static MAW_BANK: RelicTemplate = RelicTemplate {
    name: RelicName::MawBank,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
