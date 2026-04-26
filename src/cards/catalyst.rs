use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CATALYST: Entity = make_entity_card(
    CardName::Catalyst,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    true, // exhaust
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierMultiply {
            kind: ModifierKind::Poison,
            factor: 2,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded: triples instead of doubles
pub static CATALYST_PLUS: Entity = make_entity_card(
    CardName::Catalyst,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    true,
    true,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierMultiply {
            kind: ModifierKind::Poison,
            factor: 3,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
