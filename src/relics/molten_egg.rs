use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Attack Cards are obtained upgraded
// See:
//    - `process_effect_card_adopt.rs`
//    - `utils.rs`
pub static MOLTEN_EGG: Entity =
    make_entity_relic(RelicName::MoltenEgg, RelicTier::Uncommon, 0, &[]);
