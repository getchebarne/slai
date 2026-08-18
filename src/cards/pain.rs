use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static PAIN: CardTemplate = make_card_template(
    CardName::Pain,
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
