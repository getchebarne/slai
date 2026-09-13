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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
