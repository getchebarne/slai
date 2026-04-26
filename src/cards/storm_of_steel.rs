use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static STORM_OF_STEEL: Entity = make_entity_card(
    CardName::StormOfSteel,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    1,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::StormOfSteelProc { upgraded: false },
        id_source: None,
        target: Target::Direct(None),
    }],
);
// Upgraded: shivs added are upgraded
pub static STORM_OF_STEEL_PLUS: Entity = make_entity_card(
    CardName::StormOfSteel,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    1,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::StormOfSteelProc { upgraded: true },
        id_source: None,
        target: Target::Direct(None),
    }],
);
