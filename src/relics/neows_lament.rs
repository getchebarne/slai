use crate::consts::NEOW_LAMENT_COMBATS;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// 3 charges: Monsters in the next 3 combats spawn with 1 HP
// See: `process_effect_combat_start.rs`
pub static NEOWS_LAMENT: RelicTemplate = RelicTemplate {
    name: RelicName::NeowsLament,
    tier: RelicTier::Special,
    counter_init: NEOW_LAMENT_COMBATS,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
