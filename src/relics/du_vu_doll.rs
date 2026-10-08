use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat starts with 1 Strength per Curse in the deck; the counter holds the deck's Curses
// See:
//    - `process_effect_du_vu_doll_recount.rs`
//    - `process_effect_combat_start.rs`
pub static DU_VU_DOLL: RelicTemplate = RelicTemplate {
    name: RelicName::DuVuDoll,
    tier: RelicTier::Rare,
    counter_init: -1,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
