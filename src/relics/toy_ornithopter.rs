use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Using a Potion heals 5 HP
// See:
//    - `process_effect_potion_use.rs`
pub static TOY_ORNITHOPTER: Entity =
    make_entity_relic(RelicName::ToyOrnithopter, RelicTier::Common, 0, &[]);
