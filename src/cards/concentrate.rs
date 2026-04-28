use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CONCENTRATE: Entity = make_entity_card(
    CardName::Concentrate,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 3 },
            },
        },
        Effect {
            kind: EffectKind::EnergyGain { amount: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
// Upgraded
pub static CONCENTRATE_PLUS: Entity = make_entity_card(
    CardName::Concentrate,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 2 }, // -1 discard
            },
        },
        Effect {
            kind: EffectKind::EnergyGain { amount: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
