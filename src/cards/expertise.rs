use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Expertise: draw cards until your hand has up to N (6 base / 7 upgraded).
// DrawUpTo computes the deficit and pushes a CardDraw for the remainder.
pub static EXPERTISE: Entity = make_entity_card(
    CardName::Expertise,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DrawUpTo { target: 6 },
        id_source: None,
        target: Target::Direct(None),
    }],
    PlayRestriction::Always,
);
// Upgraded: draw up to 7 instead of 6
pub static EXPERTISE_PLUS: Entity = make_entity_card(
    CardName::Expertise,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DrawUpTo { target: 7 },
        id_source: None,
        target: Target::Direct(None),
    }],
    PlayRestriction::Always,
);
