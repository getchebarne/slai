use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Attacks that cost 0 deal 4 additional damage
// See:
//    - `process_effect_card_play.rs`
pub static WRIST_BLADE: RelicTemplate = RelicTemplate {
    name: RelicName::WristBlade,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
