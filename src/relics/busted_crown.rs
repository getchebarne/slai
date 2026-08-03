use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; combat Card rewards offer 2 fewer Cards
// See:
//    - `process_effect_combat_start.rs`
//    - `utils.rs`
pub static BUSTED_CROWN: Entity =
    make_entity_relic(RelicName::BustedCrown, RelicTier::Boss, 0, &[]);
