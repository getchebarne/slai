use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static WELL_LAID_PLANS: Entity = make_entity_card(
    CardName::WellLaidPlans, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Retain,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static WELL_LAID_PLANS_PLUS: Entity = make_entity_card(
    CardName::WellLaidPlans, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, true, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Retain,
            stacks: 2,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
