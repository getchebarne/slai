use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Draw 2 additional cards each turn; drawn cards get a random cost (0-3)
// See:
//    - `process_effect_turn_start.rs`
//    - `process_effect_card_draw.rs`
pub static SNECKO_EYE: Entity = make_entity_relic(RelicName::SneckoEye, RelicTier::Boss, 0, &[]);
