use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Slimed: Status card injected into the player's discard pile by the Acid /
// Spike Slime Medium and Slime Boss. Cost 1 energy to play; on play, exhausts.
// No other effect — it's a tax (one card slot in hand or one energy to clear).
// Not ethereal — it sticks around in hand until played or discarded.
pub static SLIMED: Entity = make_entity_card(
    CardName::Slimed,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Special,
    1,
    CardCostKind::Fixed,
    false, // upgraded
    true,  // exhaust on play
    false, // ethereal
    false, // innate
    false, // requires_target
    &[],
    &[],
    &[],
    PlayRestriction::Always,
);
