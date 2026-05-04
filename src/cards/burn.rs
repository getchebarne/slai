use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BURN: Entity = make_entity_card(
    CardName::Burn,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
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
    true,
    false,
    false,
    false,
    false,
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
