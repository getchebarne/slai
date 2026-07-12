use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Skill cards are obtained upgraded
pub static TOXIC_EGG: Entity =
    make_entity_relic(RelicName::ToxicEgg, RelicTier::Uncommon, 0, &[]);
