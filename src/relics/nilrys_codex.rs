use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// End of turn: discover a Card and shuffle it into the draw pile
// See:
//    - `process_effect_turn_end.rs`
pub static NILRYS_CODEX: Entity =
    make_entity_relic(RelicName::NilrysCodex, RelicTier::Special, 0, &[]);
