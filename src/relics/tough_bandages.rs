use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Manually discarding a card grants 3 block
// See:
//    - `process_effect_card_discard.rs`
pub static TOUGH_BANDAGES: Entity =
    make_entity_relic(RelicName::ToughBandages, RelicTier::Rare, 0, &[]);
