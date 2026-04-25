use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BACKSTAB: Entity = make_entity_card(
    CardName::Backstab, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    0, false, true, true, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 11 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static BACKSTAB_PLUS: Entity = make_entity_card(
    CardName::Backstab, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    0, true, true, true, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 15 }, // +4 damage
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
