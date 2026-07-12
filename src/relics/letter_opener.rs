use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd Skill played each turn deals 5 damage to all enemies
// See:
//    - `process_effect_card_play.rs`
pub static LETTER_OPENER: Entity =
    make_entity_relic(RelicName::LetterOpener, RelicTier::Uncommon, 0, &[]);
