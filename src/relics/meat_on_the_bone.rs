use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Ending combat at half HP or less heals 12 HP
pub static MEAT_ON_THE_BONE: Entity =
    make_entity_relic(RelicName::MeatOnTheBone, RelicTier::Uncommon, 0, &[]);
