use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Toke at rest sites: purge a Card from the deck
// See:
//    - `action.rs`
pub static PEACE_PIPE: RelicTemplate = RelicTemplate {
    name: RelicName::PeacePipe,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
