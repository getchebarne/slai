use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const fn make_curse(name: CardName) -> Entity {
    make_entity_card(
        name,
        CardKind::Curse,
        CardColor::Curse,
        CardRarity::Curse,
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
        PlayRestriction::Never,
    )
}
// TODO: implement
pub static ASCENDERS_BANE: Entity = make_curse(CardName::AscendersBane);
pub static REGRET: Entity = make_curse(CardName::Regret);
pub static PAIN: Entity = make_curse(CardName::Pain);
pub static DOUBT: Entity = make_curse(CardName::Doubt);
pub static DECAY: Entity = make_curse(CardName::Decay);
pub static INJURY: Entity = make_curse(CardName::Injury);
pub static SHAME: Entity = make_curse(CardName::Shame);
pub static WRITHE: Entity = make_curse(CardName::Writhe);
pub static PARASITE: Entity = make_curse(CardName::Parasite);
pub static NORMALITY: Entity = make_curse(CardName::Normality);
