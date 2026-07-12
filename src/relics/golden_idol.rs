use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_reward_roll_combat.rs`
pub static GOLDEN_IDOL: Entity =
    make_entity_relic(RelicName::GoldenIdol, RelicTier::Special, 0, &[]);
