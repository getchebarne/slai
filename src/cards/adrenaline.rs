use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ADRENALINE: Entity = card_entity(
    CardName::Adrenaline, CardKind::Skill, CardColor::Green, CardRarity::Rare,
    0, false, true, false, false,
    &[
        Effect {
            kind: EffectKind::EnergyGain { amount: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
// Upgraded
pub static ADRENALINE_PLUS: Entity = card_entity(
    CardName::Adrenaline, CardKind::Skill, CardColor::Green, CardRarity::Rare,
    0, true, true, false, false,
    &[
        Effect {
            kind: EffectKind::EnergyGain { amount: 2 }, // +1 energy gain
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
