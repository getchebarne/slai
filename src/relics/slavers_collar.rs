use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy during elite and boss fights
// See:
//    - `process_effect_combat_start.rs`
pub static SLAVERS_COLLAR: Entity =
    make_entity_relic(RelicName::SlaversCollar, RelicTier::Boss, 0, &[]);
