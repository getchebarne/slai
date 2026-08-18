use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Curses are playable; playing one costs 1 HP and exhausts it
// See:
//    - `entity.rs`
//    - `process_effect_card_play.rs`
pub static BLUE_CANDLE: RelicTemplate = RelicTemplate {
    name: RelicName::BlueCandle,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
