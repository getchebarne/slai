use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, transform all Strikes and Defends
// See:
//    - `process_effect_relic_adopt.rs`
pub static PANDORAS_BOX: RelicTemplate = RelicTemplate {
    name: RelicName::PandorasBox,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
