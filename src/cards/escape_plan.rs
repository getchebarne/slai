use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ESCAPE_PLAN: Entity = make_entity_card(
    CardName::EscapePlan,
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
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::EscapePlanCheck { block: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded: 5 block instead of 3
pub static ESCAPE_PLAN_PLUS: Entity = make_entity_card(
    CardName::EscapePlan,
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
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::EscapePlanCheck { block: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
