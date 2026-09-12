use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Inert; granted by the Bonfire Spirits event for offering a Curse
pub static SPIRIT_POOP: RelicTemplate = RelicTemplate {
    name: RelicName::SpiritPoop,
    tier: RelicTier::Special,
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
