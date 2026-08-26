use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th Attack played grants 1 energy; counter persists across turns and combats
// See:
//    - `process_effect_card_play.rs`
pub static NUNCHAKU: RelicTemplate = RelicTemplate {
    name: RelicName::Nunchaku,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
