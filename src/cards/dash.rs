use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DASH: Entity = card_entity(
    CardName::Dash, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    2, false, false, false, true,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 10 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 10 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded
pub static DASH_PLUS: Entity = card_entity(
    CardName::Dash, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    2, true, false, false, true,
    &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 13, // +3 damage
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                base: 13, // +3 block
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
