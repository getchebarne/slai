use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Inert (flavor only); a Face Trader trade outcome
pub static CULTIST_HEADPIECE: Entity =
    make_entity_relic(RelicName::CultistHeadpiece, RelicTier::Special, 0, &[]);
