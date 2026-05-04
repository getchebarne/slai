use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SLIMED: Entity = make_entity_card(
    CardName::Slimed,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Special,
    1,
    CardCostKind::Fixed,
    false,
    true, // Exhaust on play
    false,
    false,
    false,
    &[], // Does nothing
    &[],
    &[],
    PlayRestriction::Always,
);
