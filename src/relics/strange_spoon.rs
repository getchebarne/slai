use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Cards that would exhaust on play are discarded instead 50% of the time
// See:
//    - `process_effect_card_play.rs`
pub static STRANGE_SPOON: Entity =
    make_entity_relic(RelicName::StrangeSpoon, RelicTier::Shop, 0, &[]);
