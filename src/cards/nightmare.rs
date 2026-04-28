use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static NIGHTMARE: Entity = make_entity_card(
    CardName::Nightmare,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    3,
    false,
    true, // exhaust
    false,
    false,
    &[Effect {
        kind: EffectKind::CardNightmarePick { count: 3 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Hand,
            selection: SelectionKind::Input { count: 1 },
        },
    }],
    PlayRestriction::Always,
);
// Upgraded: cost 3 -> 2 (count unchanged at 3)
pub static NIGHTMARE_PLUS: Entity = make_entity_card(
    CardName::Nightmare,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    2,
    true,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardNightmarePick { count: 3 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Hand,
            selection: SelectionKind::Input { count: 1 },
        },
    }],
    PlayRestriction::Always,
);
