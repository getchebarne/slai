use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing an Attack, a Skill, and a Power in one turn removes all debuffs;
// relic_counter is a seen-kinds bitmask (Attack=1, Skill=2, Power=4)
// See:
//    - `process_effect_card_play.rs`
pub static ORANGE_PELLETS: Entity =
    make_entity_relic(RelicName::OrangePellets, RelicTier::Shop, 0, &[],
    "Whenever you play a Power, Attack, and Skill in the same turn, remove all of your Debuffs.",
);
