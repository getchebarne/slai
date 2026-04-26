use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SNEAKY_STRIKE: Entity = make_entity_card(
    CardName::SneakyStrike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    2,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::SneakyStrikeProc { energy: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
// Upgraded: +4 damage. Energy bonus unchanged.
pub static SNEAKY_STRIKE_PLUS: Entity = make_entity_card(
    CardName::SneakyStrike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    2,
    true,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 16 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::SneakyStrikeProc { energy: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
