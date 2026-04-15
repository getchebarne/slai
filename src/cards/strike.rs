use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static STRIKE: Entity = card_entity(
    CardName::Strike, CardKind::Attack, CardColor::Green, CardRarity::Basic,
    1, false, false, false, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { base: 6 },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static STRIKE_PLUS: Entity = card_entity(
    CardName::Strike, CardKind::Attack, CardColor::Green, CardRarity::Basic,
    1, true, false, false, true,
    &[Effect {
        kind: EffectKind::DamagePhysical { base: 9 }, // +3 damage
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
);
