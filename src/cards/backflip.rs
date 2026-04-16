use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BACKFLIP: Entity = make_entity_card(
    CardName::Backflip, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, false, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
// Upgraded
pub static BACKFLIP_PLUS: Entity = make_entity_card(
    CardName::Backflip, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, true, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 8, // +3 block
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
