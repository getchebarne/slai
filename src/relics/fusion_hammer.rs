use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; Smith (Card upgrade) is no longer available at rest sites
// See:
//    - `process_effect_combat_start.rs`
//    - `action.rs`
pub static FUSION_HAMMER: Entity =
    make_entity_relic(RelicName::FusionHammer, RelicTier::Boss, 0, &[]);
