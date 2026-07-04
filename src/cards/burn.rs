use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BURN: Entity = make_entity_card(
    CardName::Burn,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Common,
    0,
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
    0,
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
