use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first lethal blow leaves the character at half max HP instead
pub static LIZARD_TAIL: Entity =
    make_entity_relic(RelicName::LizardTail, RelicTier::Rare, 0, &[]);
