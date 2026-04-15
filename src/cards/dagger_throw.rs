use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DAGGER_THROW: Entity = card_entity(
    CardName::DaggerThrow, CardKind::Attack, CardColor::Green, CardRarity::Common,
    1, false, false, false, true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 9 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
);
// Upgraded
pub static DAGGER_THROW_PLUS: Entity = card_entity(
    CardName::DaggerThrow, CardKind::Attack, CardColor::Green, CardRarity::Common,
    1, true, false, false, true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 12 }, // +3 damage
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
);
