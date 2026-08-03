use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Power Cards are obtained upgraded
// See:
//    - `process_effect_card_add_to_deck.rs`
//    - `utils.rs`
pub static FROZEN_EGG: Entity =
    make_entity_relic(RelicName::FrozenEgg, RelicTier::Uncommon, 0, &[]);
