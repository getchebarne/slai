use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Resting at a rest site also offers a Card reward
// See:
//    - `action.rs`
//    - `process_effect_reward_roll_cards.rs`
pub static DREAM_CATCHER: Entity =
    make_entity_relic(RelicName::DreamCatcher, RelicTier::Common, 0, &[]);
