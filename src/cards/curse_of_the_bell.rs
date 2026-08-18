use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

// Calling Bell's curse; unpurgeable, untransformable, and never in the random curse pool
pub static CURSE_OF_THE_BELL: CardTemplate = make_card_template(
    CardName::CurseOfTheBell,
    CardKind::Curse,
    CardColor::Curse,
    CardRarity::Curse,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
