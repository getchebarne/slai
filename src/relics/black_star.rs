use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite fights drop an additional Relic
// See:
//    - `process_effect_reward_roll_combat.rs`
pub static BLACK_STAR: Entity = make_entity_relic(RelicName::BlackStar, RelicTier::Boss, 0, &[]);
