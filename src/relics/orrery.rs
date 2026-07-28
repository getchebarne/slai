use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, offers 4 card rewards (source's loop is 4 despite the card text's 5);
// relic_counter holds the rewards remaining after the first
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_room_exit.rs`
pub static ORRERY: Entity = make_entity_relic(RelicName::Orrery, RelicTier::Shop, 3, &[]);
