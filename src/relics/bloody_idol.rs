use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Heal 5 HP whenever gold is gained
// See:
//    - `process_effect_gold_delta.rs`
pub static BLOODY_IDOL: Entity =
    make_entity_relic(RelicName::BloodyIdol, RelicTier::Special, 0, &[]);
