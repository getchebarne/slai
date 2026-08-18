use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Gain 6 block whenever the discard pile is shuffled into the draw pile
// See:
//    - `process_effect_shuffle_discard_pile_into_draw_pile.rs`
pub static ABACUS: RelicTemplate = RelicTemplate {
    name: RelicName::Abacus,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
