use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd Skill played each turn deals 5 damage to all enemies
// See:
//    - `process_effect_card_play.rs`
pub static LETTER_OPENER: RelicTemplate = RelicTemplate {
    name: RelicName::LetterOpener,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
