use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static PHANTASMAL_KILLER: Entity = make_entity_card(
    CardName::PhantasmalKiller,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    1,
    false,
    false,
    false,
    false,
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
    PlayRestriction::Always,
);
// Upgraded
pub static PHANTASMAL_KILLER_PLUS: Entity = make_entity_card(
    CardName::PhantasmalKiller,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    0,
    true,
    false,
    false,
    false,
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
    PlayRestriction::Always,
);
