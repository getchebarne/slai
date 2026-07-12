use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// X-cost cards resolve with X+2; energy spent is unchanged
pub static CHEMICAL_X: Entity = make_entity_relic(RelicName::ChemicalX, RelicTier::Shop, 0, &[]);
