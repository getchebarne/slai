use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Negates the next 2 Curses added to the deck; used up at 0 charges
// See:
//    - `process_effect_card_adopt.rs`
pub static OMAMORI: RelicTemplate = RelicTemplate {
    name: RelicName::Omamori,
    tier: RelicTier::Common,
    counter_init: 2,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
