use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BACKFLIP: Entity = card_entity(
    CardName::Backflip, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, false, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 5 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            source: None,
            target: Target::Direct(None),
        },
    ],
);
// Upgraded
pub static BACKFLIP_PLUS: Entity = card_entity(
    CardName::Backflip, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, true, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 8, // +3 block
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            source: None,
            target: Target::Direct(None),
        },
    ],
);
