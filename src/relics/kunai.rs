use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_card_play.rs`
pub static KUNAI: Entity = make_entity_relic(RelicName::Kunai, RelicTier::Uncommon, 0, &[],
    "Every time you play 3 Attacks in a single turn, gain 1 Dexterity.",
);
