use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Obtaining a Curse raises max HP by 6 and heals 6
// See:
//    - `process_effect_card_adopt.rs`
pub static DARKSTONE_PERIAPT: Entity =
    make_entity_relic(RelicName::DarkstonePeriapt, RelicTier::Uncommon, 0, &[]);
