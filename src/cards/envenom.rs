use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ENVENOM: Entity = make_entity_card(
    CardName::Envenom,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    2,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Envenom,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static ENVENOM_PLUS: Entity = make_entity_card(
    CardName::Envenom,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    1, // -1 cost
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Envenom,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
