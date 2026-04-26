use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Piercing Wail: -X Strength to all enemies for one turn (refunded by
// Shackled at the enemy's turn end).
pub static PIERCING_WAIL: Entity = make_entity_card(
    CardName::PiercingWail,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    false,
    true, // exhaust
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
);
// Upgraded
pub static PIERCING_WAIL_PLUS: Entity = make_entity_card(
    CardName::PiercingWail,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    true,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: -8, // +2 magic
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
                stacks: 8,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
    ],
);
