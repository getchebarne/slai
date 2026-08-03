use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd reshuffle grants 2 energy; counter persists across combats
// See:
//    - `process_effect_shuffle_discard_pile_into_draw_pile.rs`
pub static SUNDIAL: Entity = make_entity_relic(RelicName::Sundial, RelicTier::Uncommon, 0, &[]);
