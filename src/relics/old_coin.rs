use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 300 gold
// See:
//    - `process_effect_relic_adopt.rs`
pub static OLD_COIN: RelicTemplate = RelicTemplate {
    name: RelicName::OldCoin,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
