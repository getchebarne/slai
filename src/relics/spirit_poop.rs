use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Inert; granted by the Bonfire Spirits event for offering a Curse
pub static SPIRIT_POOP: Entity =
    make_entity_relic(RelicName::SpiritPoop, RelicTier::Special, 0, &[]);
