use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On purchase, brews 5 Potions staged as a reward over the shop
// See:
//    - `process_effect_relic_adopt.rs`
pub static CAULDRON: Entity = make_entity_relic(RelicName::Cauldron, RelicTier::Shop, 0, &[]);
