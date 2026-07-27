use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; Rest is no longer available at rest sites
// See:
//    - `process_effect_combat_start.rs`
//    - `action.rs`
pub static COFFEE_DRIPPER: Entity =
    make_entity_relic(RelicName::CoffeeDripper, RelicTier::Boss, 0, &[]);
