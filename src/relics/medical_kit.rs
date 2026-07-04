use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Status cards are playable and exhaust when played
pub static MEDICAL_KIT: Entity =
    make_entity_relic(RelicName::MedicalKit, RelicTier::Shop, 0, &[]);
