use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Potions always drop after combat
// See:
//    - `process_effect_reward_roll_combat.rs`
pub static WHITE_BEAST_STATUE: Entity =
    make_entity_relic(RelicName::WhiteBeastStatue, RelicTier::Uncommon, 0, &[]);
