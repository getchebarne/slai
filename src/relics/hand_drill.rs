use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Breaking an enemy's block applies 2 Vulnerable
// See:
//    - `process_effect_damage_deal.rs`
pub static HAND_DRILL: Entity = make_entity_relic(RelicName::HandDrill, RelicTier::Shop, 0, &[]);
