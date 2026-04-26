use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BLADE_DANCE: Entity = make_entity_card(
    CardName::BladeDance,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ShivAdd { count: 3 },
        id_source: None,
        target: Target::Direct(None),
    }],
);
// Upgraded
pub static BLADE_DANCE_PLUS: Entity = make_entity_card(
    CardName::BladeDance,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ShivAdd { count: 4 }, // +1 shiv
        id_source: None,
        target: Target::Direct(None),
    }],
);
