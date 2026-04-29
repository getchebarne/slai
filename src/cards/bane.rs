use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BANE: Entity = make_entity_card(
    CardName::Bane,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 7 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysicalIfPoisoned { amount: 7 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BANE_PLUS: Entity = make_entity_card(
    CardName::Bane,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    true,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 10 }, // +3
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysicalIfPoisoned { amount: 10 }, // +3
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
