use crate::cards::make_entity_card;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ASCENDERS_BANE: Entity = make_entity_card(
    CardName::AscendersBane,
    CardKind::Curse,
    CardColor::Curse,
    CardRarity::Curse,
    0,
    CardCostKind::Fixed,
    false,
    false,
    true, // Ethereal
    false,
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
