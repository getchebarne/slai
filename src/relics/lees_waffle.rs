use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 7 max HP and heal to full
// See:
//    - `process_effect_relic_adopt.rs`
pub static LEES_WAFFLE: Entity =
    make_entity_relic(RelicName::LeesWaffle, RelicTier::Shop, 0, &[]);
