use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DEFLECT: Entity = card_entity(
    CardName::Deflect, CardKind::Skill, CardColor::Green, CardRarity::Common,
    0, false, false, false, false,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static DEFLECT_PLUS: Entity = card_entity(
    CardName::Deflect, CardKind::Skill, CardColor::Green, CardRarity::Common,
    0, true, false, false, false,
    &[Effect {
        kind: EffectKind::BlockGain {
            amount: 7, // +3 block
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
