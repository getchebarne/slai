use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Using a potion heals 5 HP
pub static TOY_ORNITHOPTER: Entity =
    make_entity_relic(RelicName::ToyOrnithopter, RelicTier::Common, 0, &[]);
