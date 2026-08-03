use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// An empty hand during your turn draws 1 Card
// See:
//    - `engine/mod.rs`
//    - `utils.rs`
pub static UNCEASING_TOP: Entity =
    make_entity_relic(RelicName::UnceasingTop, RelicTier::Rare, 0, &[]);
