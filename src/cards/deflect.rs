use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DEFLECT: Entity = make_entity_card(
    CardName::Deflect,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    0,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    PlayRestriction::Always,
);
// Upgraded
pub static DEFLECT_PLUS: Entity = make_entity_card(
    CardName::Deflect,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    0,
    true,
    false,
    false,
    false,
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
    PlayRestriction::Always,
);
