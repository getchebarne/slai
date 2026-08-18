use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Skill Cards are obtained upgraded
// See:
//    - `process_effect_card_adopt.rs`
//    - `utils.rs`
pub static TOXIC_EGG: RelicTemplate = RelicTemplate {
    name: RelicName::ToxicEgg,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
