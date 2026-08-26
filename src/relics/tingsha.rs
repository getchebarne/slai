use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Manually discarding a Card deals 3 damage to a random enemy
// See:
//    - `process_effect_card_discard.rs`
pub static TINGSHA: RelicTemplate = RelicTemplate {
    name: RelicName::Tingsha,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
