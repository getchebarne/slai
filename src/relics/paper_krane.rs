use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Weakened enemies deal 40 percent less damage instead of 25
// See:
//    - `process_effect_damage_physical.rs`
pub static PAPER_KRANE: Entity =
    make_entity_relic(RelicName::PaperKrane, RelicTier::Uncommon, 0, &[],
    "Enemies with Weak deal 40% less damage rather than 25%.",
);
