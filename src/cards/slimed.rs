use crate::cards::make_entity_card;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

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
