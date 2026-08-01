use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat card rewards offer 1 additional card
// See:
//    - `utils.rs`
pub static QUESTION_CARD: Entity =
    make_entity_relic(RelicName::QuestionCard, RelicTier::Uncommon, 0, &[]);
