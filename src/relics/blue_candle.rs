use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Curses are playable; playing one costs 1 HP and exhausts it
pub static BLUE_CANDLE: Entity =
    make_entity_relic(RelicName::BlueCandle, RelicTier::Uncommon, 0, &[]);
