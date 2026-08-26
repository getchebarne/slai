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
    effects_combat_start: &[],
};
