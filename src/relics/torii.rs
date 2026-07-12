use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Incoming attacks leaving a 2-5 HP remainder deal 1 instead
// See:
//    - `process_effect_damage_deal.rs`
pub static TORII: Entity =
    make_entity_relic(RelicName::Torii, RelicTier::Rare, 0, &[],
    "Whenever you would receive 5 or less unblocked Attack damage, reduce it to 1.",
);
