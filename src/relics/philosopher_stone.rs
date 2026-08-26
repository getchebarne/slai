use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; all Monsters (including mid-combat spawns) start with 1 Strength
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_monster_spawn.rs`
pub static PHILOSOPHER_STONE: RelicTemplate = RelicTemplate {
    name: RelicName::PhilosopherStone,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
