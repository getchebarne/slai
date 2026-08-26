use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, choose 3 Cards to transform; the results are upgraded
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_astrolabe_transform.rs`
pub static ASTROLABE: RelicTemplate = RelicTemplate {
    name: RelicName::Astrolabe,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
