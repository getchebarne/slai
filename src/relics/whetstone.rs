use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, upgrade 2 random Attacks
// See:
//    - `process_effect_relic_adopt.rs`
pub static WHETSTONE: Entity =
    make_entity_relic(RelicName::Whetstone, RelicTier::Common, 0, &[],
    "Upon pick up, Upgrade 2 random Attacks.",
);
