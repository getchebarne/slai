use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ALL_OUT_ATTACK: Entity = make_entity_card(
    CardName::AllOutAttack,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 10 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Random { count: 1 },
            },
        },
    ],
);
// Upgraded
pub static ALL_OUT_ATTACK_PLUS: Entity = make_entity_card(
    CardName::AllOutAttack,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 14, // +4 damage
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Random { count: 1 },
            },
        },
    ],
);
