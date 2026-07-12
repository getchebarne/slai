use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 10 max HP
// See:
//    - `process_effect_relic_adopt.rs`
pub static PEAR: Entity =
    make_entity_relic(RelicName::Pear, RelicTier::Uncommon, 0, &[]);
