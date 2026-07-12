use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Manually discarding a card deals 3 damage to a random enemy
// See:
//    - `process_effect_card_discard.rs`
pub static TINGSHA: Entity =
    make_entity_relic(RelicName::Tingsha, RelicTier::Rare, 0, &[]);
