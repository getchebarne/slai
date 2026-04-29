use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ADRENALINE: Entity = make_entity_card(
    CardName::Adrenaline,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
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
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ADRENALINE_PLUS: Entity = make_entity_card(
    CardName::Adrenaline,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    true,
    true,
    false,
    false,
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
    &[],
    &[],
    PlayRestriction::Always,
);
