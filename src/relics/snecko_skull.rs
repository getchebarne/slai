use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Applying Poison to an enemy applies 1 more
// See:
//    - `process_effect_modifier_gain.rs`
pub static SNECKO_SKULL: Entity =
    make_entity_relic(RelicName::SneckoSkull, RelicTier::Common, 0, &[],
    "Whenever you apply Poison, apply an additional 1 Poison.",
);
