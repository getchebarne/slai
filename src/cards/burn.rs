use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Burn: Status card injected into the player's discard by Hexaghost's Sear
// and Inferno. Unplayable; deals 2 damage (4 upgraded) to the character at
// end of player turn while in hand. Damage tick lives in
// process_effect_turn_end_character; Burn's static effects array is empty.
pub static BURN: Entity = make_entity_card(
    CardName::Burn,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false, // upgraded
    false, // exhaust on play (irrelevant)
    false, // ethereal
    false, // innate
    false, // requires_target
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);

pub static BURN_UPGRADED: Entity = make_entity_card(
    CardName::Burn,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    true, // upgraded
    false,
    false,
    false,
    false,
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
