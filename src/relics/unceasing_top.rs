use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// An empty hand during your turn draws 1 card
pub static UNCEASING_TOP: Entity =
    make_entity_relic(RelicName::UnceasingTop, RelicTier::Rare, 0, &[]);
