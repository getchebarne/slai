use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Using a Potion heals 5 HP
// See:
//    - `process_effect_potion_use.rs`
pub static TOY_ORNITHOPTER: RelicTemplate = RelicTemplate {
    name: RelicName::ToyOrnithopter,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
