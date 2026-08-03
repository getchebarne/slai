use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, transform all Strikes and Defends
// See:
//    - `process_effect_relic_adopt.rs`
pub static PANDORAS_BOX: Entity =
    make_entity_relic(RelicName::PandorasBox, RelicTier::Boss, 0, &[]);
