use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Negates the next 2 Curses added to the deck; used up at 0 charges
pub static OMAMORI: Entity = make_entity_relic(RelicName::Omamori, RelicTier::Common, 2, &[]);
