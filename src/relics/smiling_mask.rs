use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The shop Card-removal service always costs 50 gold
// See:
//    - `process_effect_shop_build.rs`
pub static SMILING_MASK: Entity =
    make_entity_relic(RelicName::SmilingMask, RelicTier::Common, 0, &[]);
