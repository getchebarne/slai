use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Strike-tagged cards deal 3 more damage
// See:
//    - `process_effect_damage_physical.rs`
pub static STRIKE_DUMMY: Entity =
    make_entity_relic(RelicName::StrikeDummy, RelicTier::Uncommon, 0, &[],
    "Cards containing \"Strike\" deal 3 additional damage.",
);
