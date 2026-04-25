use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DASH: Entity = make_entity_card(
    CardName::Dash, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    2, false, false, false, true,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 10 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 10 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded
pub static DASH_PLUS: Entity = make_entity_card(
    CardName::Dash, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    2, true, false, false, true,
    &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 13, // +3 damage
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 13, // +3 block
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
