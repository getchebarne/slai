use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static NEUTRALIZE: Entity = make_entity_card(
    CardName::Neutralize, CardKind::Attack, CardColor::Green, CardRarity::Basic,
    0, false, false, false, true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded
pub static NEUTRALIZE_PLUS: Entity = make_entity_card(
    CardName::Neutralize, CardKind::Attack, CardColor::Green, CardRarity::Basic,
    0, true, false, false, true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 4 }, // +1 damage
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2, // +1 stack
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
