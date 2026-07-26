use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `utils.rs::vuln_factor` (Vulnerable deals x1.25 to the character instead of x1.5)
pub static ODD_MUSHROOM: Entity =
    make_entity_relic(RelicName::OddMushroom, RelicTier::Special, 0, &[]);
