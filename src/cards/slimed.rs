use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SLIMED: CardTemplate = make_card_template(
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
    &[], // Does nothing
    &[],
    &[],
    PlayRestriction::Always,
);
