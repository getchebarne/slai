use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat Card rewards offer 1 additional Card
// See:
//    - `utils.rs`
pub static QUESTION_CARD: RelicTemplate = RelicTemplate {
    name: RelicName::QuestionCard,
    tier: RelicTier::Uncommon,
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
