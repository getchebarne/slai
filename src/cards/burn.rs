use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BURN: CardTemplate = make_card_template(
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
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);

pub static BURN_UPGRADED: CardTemplate = make_card_template(
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
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
