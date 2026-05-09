use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

// Behavior runs through inline counter logic in process_effect_card_play and
// reset in process_effect_turn_end_character. No combat-start effect
pub static SHURIKEN: Entity =
    make_entity_relic(RelicName::Shuriken, RelicTier::Rare, 0, &[]);
