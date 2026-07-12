use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// After a rest site, start the next combat with 2 extra energy
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_room_enter.rs`
pub static ANCIENT_TEA_SET: Entity =
    make_entity_relic(RelicName::AncientTeaSet, RelicTier::Common, 0, &[],
    "Whenever you enter a Rest Site, start the next combat with 2 extra Energy.",
);
