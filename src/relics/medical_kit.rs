use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Status Cards are playable and exhaust when played
// See:
//    - `entity.rs`
//    - `process_effect_card_play.rs`
pub static MEDICAL_KIT: Entity = make_entity_relic(RelicName::MedicalKit, RelicTier::Shop, 0, &[]);
