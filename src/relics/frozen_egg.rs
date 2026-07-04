use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Power cards are obtained upgraded
pub static FROZEN_EGG: Entity =
    make_entity_relic(RelicName::FrozenEgg, RelicTier::Uncommon, 0, &[]);
