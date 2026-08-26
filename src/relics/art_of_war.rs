use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// If no Attack was played this turn, gain 1 energy next turn
// See:
//    - `process_effect_turn_end.rs`
pub static ART_OF_WAR: RelicTemplate = RelicTemplate {
    name: RelicName::ArtOfWar,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
