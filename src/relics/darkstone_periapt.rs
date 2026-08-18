use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Obtaining a Curse raises max HP by 6 and heals 6
// See:
//    - `process_effect_card_adopt.rs`
pub static DARKSTONE_PERIAPT: RelicTemplate = RelicTemplate {
    name: RelicName::DarkstonePeriapt,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
