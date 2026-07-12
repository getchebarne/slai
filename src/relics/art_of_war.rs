use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// If no Attack was played this turn, gain 1 energy next turn
// See:
//    - `process_effect_turn_end.rs`
pub static ART_OF_WAR: Entity =
    make_entity_relic(RelicName::ArtOfWar, RelicTier::Common, 0, &[],
    "If you do not play any Attacks during your turn, gain an extra Energy next turn.",
);
