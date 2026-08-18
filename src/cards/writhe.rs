use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static WRITHE: CardTemplate = make_card_template(
    CardName::Writhe,
    CardKind::Curse,
    CardColor::Curse,
    CardRarity::Curse,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    true, // Innate
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
