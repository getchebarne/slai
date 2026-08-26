use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Dig at rest sites: gain a random Relic (granted directly, not staged as a reward)
// See:
//    - `action.rs`
pub static SHOVEL: RelicTemplate = RelicTemplate {
    name: RelicName::Shovel,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
