use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing an Attack, a Skill, and a Power in one turn removes all debuffs;
// relic_counter is a seen-kinds bitmask (Attack=1, Skill=2, Power=4)
// See:
//    - `process_effect_card_play.rs`
pub static ORANGE_PELLETS: RelicTemplate = RelicTemplate {
    name: RelicName::OrangePellets,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
