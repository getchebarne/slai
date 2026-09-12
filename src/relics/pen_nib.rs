use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th Attack deals double damage; counter persists across combats
// See:
//    - `process_effect_card_play.rs`
//    - `process_effect_combat_start.rs`
pub static PEN_NIB: RelicTemplate = RelicTemplate {
    name: RelicName::PenNib,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
