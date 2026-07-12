use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_relic_grant_specific.rs`
//    - `utils.rs`
pub static CIRCLET: Entity = make_entity_relic(RelicName::Circlet, RelicTier::Special, 0, &[],
    "Collect as many as you can.",
);
