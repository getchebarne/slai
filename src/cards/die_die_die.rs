use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DIE_DIE_DIE: Entity = card_entity(
    CardName::DieDieDie, CardKind::Attack, CardColor::Green, CardRarity::Rare,
    1, false, true, false, false,
    &[Effect {
        kind: EffectKind::DamagePhysical { base: 13 },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static DIE_DIE_DIE_PLUS: Entity = card_entity(
    CardName::DieDieDie, CardKind::Attack, CardColor::Green, CardRarity::Rare,
    1, true, true, false, false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            base: 17, // +4 damage
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
);
