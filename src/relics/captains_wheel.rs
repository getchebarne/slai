use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the start of turn 3 each combat, gain 18 block
pub static CAPTAINS_WHEEL: Entity =
    make_entity_relic(RelicName::CaptainsWheel, RelicTier::Rare, 0, &[]);
