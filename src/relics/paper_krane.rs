use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Weakened enemies deal 40 percent less damage instead of 25
// See:
//    - `process_effect_damage_physical.rs`
pub static PAPER_KRANE: RelicTemplate = RelicTemplate {
    name: RelicName::PaperKrane,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
