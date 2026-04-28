use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DISTRACTION: Entity = make_entity_card(
    CardName::Distraction,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    true, // exhaust
    false,
    false,
    &[Effect {
        kind: EffectKind::DistractionAdd,
        id_source: None,
        target: Target::Direct(None),
    }],
    PlayRestriction::Always,
);
// Upgraded: cost 1 -> 0
pub static DISTRACTION_PLUS: Entity = make_entity_card(
    CardName::Distraction,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    true,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::DistractionAdd,
        id_source: None,
        target: Target::Direct(None),
    }],
    PlayRestriction::Always,
);
