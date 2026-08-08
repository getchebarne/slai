use crate::consts::NEOW_LAMENT_COMBATS;
use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// 3 charges: Monsters in the next 3 combats spawn with 1 HP
// See: `process_effect_combat_start.rs`
pub static NEOWS_LAMENT: Entity = make_entity_relic(
    RelicName::NeowsLament,
    RelicTier::Special,
    NEOW_LAMENT_COMBATS,
    &[],
);
