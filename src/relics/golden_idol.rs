use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

pub static GOLDEN_IDOL: Entity =
    make_entity_relic(RelicName::GoldenIdol, RelicTier::Special, 0, &[]);
