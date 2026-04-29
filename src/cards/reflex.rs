use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const ON_DISCARD: &[Effect] = &[Effect {
    kind: EffectKind::CardDraw { count: 2 },
    id_source: None,
    target: Target::Direct(None),
}];
const ON_DISCARD_PLUS: &[Effect] = &[Effect {
    kind: EffectKind::CardDraw { count: 3 }, // +1 draw
    id_source: None,
    target: Target::Direct(None),
}];

pub static REFLEX: Entity = Entity {
    card_on_discard_effects: ON_DISCARD,
    ..make_entity_card(
        CardName::Reflex,
        CardKind::Skill,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        false,
        false,
        false,
        false,
        &[],
        PlayRestriction::Never,
    )
};
// Upgraded
pub static REFLEX_PLUS: Entity = Entity {
    card_on_discard_effects: ON_DISCARD_PLUS,
    ..make_entity_card(
        CardName::Reflex,
        CardKind::Skill,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        true,
        false,
        false,
        false,
        &[],
        PlayRestriction::Never,
    )
};
