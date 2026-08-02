use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Potion effects are doubled (discover Potions excluded)
// See:
//    - `process_effect_potion_use.rs`
pub static SACRED_BARK: Entity = make_entity_relic(RelicName::SacredBark, RelicTier::Boss, 0, &[]);
