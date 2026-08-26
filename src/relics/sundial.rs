use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd reshuffle grants 2 energy; counter persists across combats
// See:
//    - `process_effect_shuffle_discard_pile_into_draw_pile.rs`
pub static SUNDIAL: RelicTemplate = RelicTemplate {
    name: RelicName::Sundial,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
