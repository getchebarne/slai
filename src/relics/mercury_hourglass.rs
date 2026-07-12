use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every turn start deals 3 damage to all enemies
// See:
//    - `process_effect_turn_start.rs`
pub static MERCURY_HOURGLASS: Entity =
    make_entity_relic(RelicName::MercuryHourglass, RelicTier::Uncommon, 0, &[],
    "At the start of your turn, deal 3 damage to ALL enemies.",
);
