use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 14 max HP
// See:
//    - `process_effect_relic_adopt.rs`
pub static MANGO: Entity =
    make_entity_relic(RelicName::Mango, RelicTier::Rare, 0, &[],
    "Upon pickup, raise your Max HP by 14.",
);
