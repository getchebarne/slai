use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; all monsters (including mid-combat spawns) start with 1 Strength
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_monster_spawn.rs`
pub static PHILOSOPHER_STONE: Entity =
    make_entity_relic(RelicName::PhilosopherStone, RelicTier::Boss, 0, &[]);
