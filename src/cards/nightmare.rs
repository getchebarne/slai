use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static NIGHTMARE: Entity = make_entity_card(
    CardName::Nightmare,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    3,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardNightmarePick,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Hand,
            selection: SelectionKind::Input { count: 1 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static NIGHTMARE_PLUS: Entity = make_entity_card(
    CardName::Nightmare,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    2, // -1 cost
    CardCostKind::Fixed,
    true,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardNightmarePick,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Hand,
            selection: SelectionKind::Input { count: 1 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
