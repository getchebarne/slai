use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 300 gold
pub static OLD_COIN: Entity =
    make_entity_relic(RelicName::OldCoin, RelicTier::Rare, 0, &[]);
