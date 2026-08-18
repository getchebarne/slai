use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the end of turn 7 each combat, deal 52 damage to all enemies
// See:
//    - `process_effect_turn_end.rs`
pub static STONE_CALENDAR: RelicTemplate = RelicTemplate {
    name: RelicName::StoneCalendar,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
