use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static FINISHER: Entity = make_entity_card(
    CardName::Finisher,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::FinisherDamage { damage_per: 6 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static FINISHER_PLUS: Entity = make_entity_card(
    CardName::Finisher,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::FinisherDamage { damage_per: 8 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
