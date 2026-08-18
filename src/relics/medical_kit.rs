use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Status Cards are playable and exhaust when played
// See:
//    - `entity.rs`
//    - `process_effect_card_play.rs`
pub static MEDICAL_KIT: RelicTemplate = RelicTemplate {
    name: RelicName::MedicalKit,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
