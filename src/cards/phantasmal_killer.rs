use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static PHANTASMAL_KILLER: Entity = card_entity(
    CardName::PhantasmalKiller, CardKind::Skill, CardColor::Green, CardRarity::Rare,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Phantasmal,
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
pub static PHANTASMAL_KILLER_PLUS: Entity = card_entity(
    CardName::PhantasmalKiller, CardKind::Skill, CardColor::Green, CardRarity::Rare,
    0, true, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Phantasmal,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
