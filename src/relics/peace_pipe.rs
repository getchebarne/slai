use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Toke at rest sites: purge a card from the deck
// See:
//    - `action.rs`
pub static PEACE_PIPE: Entity = make_entity_relic(RelicName::PeacePipe, RelicTier::Rare, 0, &[]);
