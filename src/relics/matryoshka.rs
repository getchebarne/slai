use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The next 2 chests contain an extra Relic
// See:
//    - `process_effect_reward_roll_chest.rs`
pub static MATRYOSHKA: Entity =
    make_entity_relic(RelicName::Matryoshka, RelicTier::Uncommon, 2, &[]);
