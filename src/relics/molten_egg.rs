use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Attack cards are obtained upgraded
pub static MOLTEN_EGG: Entity =
    make_entity_relic(RelicName::MoltenEgg, RelicTier::Uncommon, 0, &[]);
