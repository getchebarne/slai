use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first Attack costing 2+ each turn is played twice; grants Necronomicurse on pickup
// See:
//    - `process_effect_card_play.rs`
//    - `process_effect_relic_adopt.rs`
pub static NECRONOMICON: Entity =
    make_entity_relic(RelicName::Necronomicon, RelicTier::Special, 0, &[]);
