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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
