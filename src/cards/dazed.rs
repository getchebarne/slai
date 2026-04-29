use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Dazed: Status card injected into the player's discard pile by Sentry's
// Bolt move. Unplayable; ethereal — auto-exhausts at end of turn if still
// in hand. Cost field is irrelevant since PlayRestriction::Never blocks
// the play.
pub static DAZED: Entity = make_entity_card(
    CardName::Dazed,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Special,
    0,
    CardCostKind::Fixed,
    false, // upgraded
    false, // exhaust on play (irrelevant; can't be played)
    true,  // ethereal — auto-exhaust at end of turn
    false, // innate
    false, // requires_target
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
