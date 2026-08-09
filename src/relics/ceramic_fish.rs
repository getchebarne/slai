use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Adding a Card to the deck grants 9 gold
// See:
//    - `process_effect_card_add_to_deck.rs`
pub static CERAMIC_FISH: Entity =
    make_entity_relic(RelicName::CeramicFish, RelicTier::Common, 0, &[]);
