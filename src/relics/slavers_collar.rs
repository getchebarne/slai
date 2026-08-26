use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy during elite and boss fights
// See:
//    - `process_effect_combat_start.rs`
pub static SLAVERS_COLLAR: RelicTemplate = RelicTemplate {
    name: RelicName::SlaversCollar,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
