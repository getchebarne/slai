use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// End of turn: discover a Card and shuffle it into the draw pile
// See:
//    - `process_effect_turn_end.rs`
pub static NILRYS_CODEX: RelicTemplate = RelicTemplate {
    name: RelicName::NilrysCodex,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
