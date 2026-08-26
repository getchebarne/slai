use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DAZED: CardTemplate = make_card_template(
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
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
