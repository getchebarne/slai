use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DIE_DIE_DIE: Entity = make_entity_card(
    CardName::DieDieDie, CardKind::Attack, CardColor::Green, CardRarity::Rare,
    1, false, true, false, false,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 13 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static DIE_DIE_DIE_PLUS: Entity = make_entity_card(
    CardName::DieDieDie, CardKind::Attack, CardColor::Green, CardRarity::Rare,
    1, true, true, false, false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 17, // +4 damage
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
);
