use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DISTRACTION: Entity = make_entity_card(
    CardName::Distraction,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DistractionAdd,
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static DISTRACTION_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost: 0, // -1 cost
    ..DISTRACTION
};
