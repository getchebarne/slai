use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; Potions can no longer be obtained
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_potion_adopt.rs`
//    - `process_effect_shop_buy_potion.rs`
pub static SOZU: RelicTemplate = RelicTemplate {
    name: RelicName::Sozu,
    tier: RelicTier::Boss,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
