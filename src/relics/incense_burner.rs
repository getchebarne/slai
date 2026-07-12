use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 6th turn start grants 1 Intangible; counter persists across combats
// See:
//    - `process_effect_turn_start.rs`
pub static INCENSE_BURNER: Entity =
    make_entity_relic(RelicName::IncenseBurner, RelicTier::Rare, 0, &[],
    "Every 6 turns, gain 1 Intangible.",
);
