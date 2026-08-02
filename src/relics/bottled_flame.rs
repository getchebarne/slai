use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, bottle an Attack; it starts every combat in the opening hand
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_card_bottle.rs`
//    - `process_effect_combat_start.rs`
pub static BOTTLED_FLAME: Entity =
    make_entity_relic(RelicName::BottledFlame, RelicTier::Uncommon, 0, &[]);
