use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SHIV: Entity = card_entity(
    CardName::Shiv, CardKind::Attack, CardColor::Colorless, CardRarity::Special,
    0, false, true, false, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { base: 4 },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static SHIV_PLUS: Entity = card_entity(
    CardName::Shiv, CardKind::Attack, CardColor::Colorless, CardRarity::Special,
    0, true, true, false, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { base: 6 }, // +2 damage
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
