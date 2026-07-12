use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Obtaining a Curse raises max HP by 6 and heals 6
// See:
//    - `process_effect_card_add_to_deck.rs`
pub static DARKSTONE_PERIAPT: Entity =
    make_entity_relic(RelicName::DarkstonePeriapt, RelicTier::Uncommon, 0, &[],
    "Whenever you obtain a Curse, increase your Max HP by 6.",
);
