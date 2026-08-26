use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing a Power heals 2 HP
// See:
//    - `process_effect_card_play.rs`
pub static BIRD_FACED_URN: RelicTemplate = RelicTemplate {
    name: RelicName::BirdFacedUrn,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
