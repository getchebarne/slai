use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Power cards are obtained upgraded
// See:
//    - `process_effect_card_add_to_deck.rs`
//    - `utils.rs`
pub static FROZEN_EGG: Entity =
    make_entity_relic(RelicName::FrozenEgg, RelicTier::Uncommon, 0, &[],
    "Whenever you add a Power card to your deck, it is Upgraded.",
);
