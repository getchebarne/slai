use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DEADLY_POISON: Entity = make_entity_card(
    CardName::DeadlyPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 5,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static DEADLY_POISON_PLUS: Entity = make_entity_card(
    CardName::DeadlyPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 7, // +2
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
