use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing a Power makes a random cost>0 Card in hand free for the turn
// See:
//    - `process_effect_card_play.rs`
pub static MUMMIFIED_HAND: RelicTemplate = RelicTemplate {
    name: RelicName::MummifiedHand,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
