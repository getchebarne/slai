use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The next 2 chests contain an extra Relic
// See:
//    - `process_effect_reward_roll_chest.rs`
pub static MATRYOSHKA: RelicTemplate = RelicTemplate {
    name: RelicName::Matryoshka,
    tier: RelicTier::Uncommon,
    counter_init: 2,
    effects_combat_start: &[],
};
