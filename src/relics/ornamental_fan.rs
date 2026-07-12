use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd Attack played each turn grants 4 block
// See:
//    - `process_effect_card_play.rs`
pub static ORNAMENTAL_FAN: Entity =
    make_entity_relic(RelicName::OrnamentalFan, RelicTier::Uncommon, 0, &[]);
