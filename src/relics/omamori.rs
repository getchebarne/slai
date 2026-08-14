use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Negates the next 2 Curses added to the deck; used up at 0 charges
// See:
//    - `process_effect_card_adopt.rs`
pub static OMAMORI: Entity = make_entity_relic(RelicName::Omamori, RelicTier::Common, 2, &[]);
