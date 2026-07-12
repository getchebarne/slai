use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 2 potion slots
// See:
//    - `process_effect_relic_adopt.rs`
pub static POTION_BELT: Entity =
    make_entity_relic(RelicName::PotionBelt, RelicTier::Common, 0, &[]);
