use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DEFEND: Entity = card_entity(
    CardName::Defend, CardKind::Skill, CardColor::Green, CardRarity::Basic,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 5 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static DEFEND_PLUS: Entity = card_entity(
    CardName::Defend, CardKind::Skill, CardColor::Green, CardRarity::Basic,
    1, true, false, false, false,
    &[Effect {
        kind: EffectKind::BlockGain {
            amount: 8, // +3 block
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
