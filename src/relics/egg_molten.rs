use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Attack Cards are obtained upgraded
// See:
//    - `process_effect_card_adopt.rs`
//    - `utils.rs`
pub static EGG_MOLTEN: RelicTemplate = RelicTemplate {
    name: RelicName::EggMolten,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
