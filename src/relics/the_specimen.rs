use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// An enemy dying with Poison passes it to a random enemy
// See:
//    - `process_effect_death.rs`
pub static THE_SPECIMEN: Entity =
    make_entity_relic(RelicName::TheSpecimen, RelicTier::Rare, 0, &[],
    "Whenever an enemy dies, transfer any Poison it has to a random enemy.",
);
