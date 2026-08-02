use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; gold can no longer be gained
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_gold_delta.rs`
//    - `process_effect_combat_end.rs`
pub static ECTOPLASM: Entity = make_entity_relic(RelicName::Ectoplasm, RelicTier::Boss, 0, &[]);
