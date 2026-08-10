use crate::cards::make_entity_card;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static WOUND: Entity = make_entity_card(
    CardName::Wound,
    CardKind::Status,
    CardColor::Colorless,
    CardRarity::Special,
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
    PlayRestriction::Never, // Unplayable
);
