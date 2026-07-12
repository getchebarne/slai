use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing a Power makes a random cost>0 card in hand free for the turn
// See:
//    - `process_effect_card_play.rs`
pub static MUMMIFIED_HAND: Entity =
    make_entity_relic(RelicName::MummifiedHand, RelicTier::Uncommon, 0, &[]);
