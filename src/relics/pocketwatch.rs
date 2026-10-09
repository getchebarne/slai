use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing 3 or fewer Cards in a turn draws 3 extra Cards next turn; the counter holds the turn's plays
// See:
//    - `process_card_play.rs`
//    - `process_effect_turn_start_character.rs`
pub static POCKETWATCH: RelicTemplate = RelicTemplate {
    name: RelicName::Pocketwatch,
    tier: RelicTier::Rare,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
