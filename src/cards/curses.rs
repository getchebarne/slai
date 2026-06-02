use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

const fn make_curse(name: CardName, ethereal: bool, innate: bool) -> Entity {
    make_entity_card(
        name,
        CardKind::Curse,
        CardColor::Curse,
        CardRarity::Curse,
        0,
        CardCostKind::Fixed,
        false, // upgraded
        false, // exhaust
        ethereal,
        innate,
        false, // requires_target
        &[],
        &[],
        &[],
        PlayRestriction::Never,
    )
}
pub static ASCENDERS_BANE: Entity = make_curse(CardName::AscendersBane, true, false);
pub static REGRET: Entity = make_curse(CardName::Regret, false, false);
pub static PAIN: Entity = make_curse(CardName::Pain, false, false);
pub static DOUBT: Entity = make_curse(CardName::Doubt, false, false);
pub static DECAY: Entity = make_curse(CardName::Decay, false, false);
pub static INJURY: Entity = make_curse(CardName::Injury, false, false);
pub static SHAME: Entity = make_curse(CardName::Shame, false, false);
pub static WRITHE: Entity = make_curse(CardName::Writhe, false, true);
pub static PARASITE: Entity = make_curse(CardName::Parasite, false, false);
pub static NORMALITY: Entity = make_curse(CardName::Normality, false, false);
