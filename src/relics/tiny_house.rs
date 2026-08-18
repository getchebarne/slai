use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup: upgrade 1 random Card, +5 max HP (healed), 50 gold, 1 random Potion
// See:
//    - `process_effect_relic_adopt.rs`
pub static TINY_HOUSE: RelicTemplate = RelicTemplate {
    name: RelicName::TinyHouse,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
