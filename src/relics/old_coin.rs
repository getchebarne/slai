use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 300 gold
// See:
//    - `process_effect_relic_adopt.rs`
pub static OLD_COIN: Entity = make_entity_relic(RelicName::OldCoin, RelicTier::Rare, 0, &[]);
