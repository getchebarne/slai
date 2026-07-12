use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th Attack played grants 1 energy; counter persists across turns and combats
pub static NUNCHAKU: Entity = make_entity_relic(RelicName::Nunchaku, RelicTier::Common, 0, &[]);
