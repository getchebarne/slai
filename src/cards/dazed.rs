use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DAZED: Entity = make_entity_card(
    CardName::Dazed,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Special,
    0,
    CardCostKind::Fixed,
    false,
    false,
    true, // Ethereal — auto-exhaust at end of turn
    false,
    false,
    &[],
    &[],
    &[],
    PlayRestriction::Never, // Unplayable
    &[],
);
