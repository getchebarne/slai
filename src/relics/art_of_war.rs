use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// If no Attack was used this turn, gain 1 energy next turn; the counter latches 1 at the turn's first Attack
// See:
//    - `process_card_play.rs`
//    - `process_effect_turn_start_character.rs`
pub static ART_OF_WAR: RelicTemplate = RelicTemplate {
    name: RelicName::ArtOfWar,
    tier: RelicTier::Common,
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
