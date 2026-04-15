use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BURST: Entity = card_entity(
    CardName::Burst, CardKind::Skill, CardColor::Green, CardRarity::Rare,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Burst,
            stacks: 1,
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static BURST_PLUS: Entity = card_entity(
    CardName::Burst, CardKind::Skill, CardColor::Green, CardRarity::Rare,
    1, true, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Burst,
            stacks: 2, // +1 stack
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
