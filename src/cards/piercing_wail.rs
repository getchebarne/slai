use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static PIERCING_WAIL: Entity = make_entity_card(
    CardName::PiercingWail,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    true, 
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: -6,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Shackled,
                stacks: 6,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PIERCING_WAIL_PLUS: Entity = make_entity_card(
    CardName::PiercingWail,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    true,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: -8, // -2 strength
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Shackled,
                stacks: 8, // +2 strength
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
