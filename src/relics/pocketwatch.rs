use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing 3 or fewer Cards in a turn draws 3 extra Cards next turn
// See:
//    - `process_effect_turn_end.rs`
pub static POCKETWATCH: Entity = make_entity_relic(RelicName::Pocketwatch, RelicTier::Rare, 0, &[]);
