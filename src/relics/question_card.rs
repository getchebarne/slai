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
    effects_combat_start: &[],
};
