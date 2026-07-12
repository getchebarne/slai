use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the end of turn 7 each combat, deal 52 damage to all enemies
// See:
//    - `process_effect_turn_end.rs`
pub static STONE_CALENDAR: Entity =
    make_entity_relic(RelicName::StoneCalendar, RelicTier::Rare, 0, &[],
    "At the end of turn 7, deal 52 damage to ALL enemies.",
);
