use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; Potions can no longer be obtained
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_potion_adopt.rs`
//    - `process_effect_shop_buy_potion.rs`
pub static SOZU: Entity = make_entity_relic(RelicName::Sozu, RelicTier::Boss, 0, &[]);
