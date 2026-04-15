use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BACKSTAB: Entity = card_entity(
    CardName::Backstab, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    0, false, true, true, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { base: 11 },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static BACKSTAB_PLUS: Entity = card_entity(
    CardName::Backstab, CardKind::Attack, CardColor::Green, CardRarity::Uncommon,
    0, true, true, true, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { base: 15 }, // +4 damage
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
