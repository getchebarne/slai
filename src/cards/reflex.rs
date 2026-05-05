use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static REFLEX: Entity = make_entity_card(
    CardName::Reflex,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[],
    &[Effect {
        kind: EffectKind::CardDraw { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    PlayRestriction::Never,
);
// Upgraded
pub static REFLEX_PLUS: Entity = Entity {
    card_upgraded: true,
    card_on_discard_effects: &[Effect {
        kind: EffectKind::CardDraw { count: 3 }, // +1 draw
        id_source: None,
        target: Target::Direct(None),
    }],
    ..REFLEX
};
