use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first Card discarded each turn grants 1 energy
// See:
//    - `process_effect_card_discard.rs`
pub static HOVERING_KITE: Entity =
    make_entity_relic(RelicName::HoveringKite, RelicTier::Boss, 0, &[]);
