use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// At combat start, choose 1 of 3 colorless Cards to add to the hand
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_toolbox_roll.rs`
//    - `process_effect_toolbox_pick.rs`
pub static TOOLBOX: Entity = make_entity_relic(RelicName::Toolbox, RelicTier::Shop, 0, &[]);
