use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Resting at a rest site also offers a Card reward
// See:
//    - `action.rs`
//    - `process_effect_reward_roll_cards.rs`
pub static DREAM_CATCHER: RelicTemplate = RelicTemplate {
    name: RelicName::DreamCatcher,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
