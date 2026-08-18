use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th Card played draws 1 Card; counter persists across turns and combats
// See:
//    - `process_effect_card_play.rs`
pub static INK_BOTTLE: RelicTemplate = RelicTemplate {
    name: RelicName::InkBottle,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
