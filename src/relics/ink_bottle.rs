use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th card played draws 1 card; counter persists across turns and combats
// See:
//    - `process_effect_card_play.rs`
pub static INK_BOTTLE: Entity =
    make_entity_relic(RelicName::InkBottle, RelicTier::Uncommon, 0, &[]);
