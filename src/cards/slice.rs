use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SLICE: Entity = make_entity_card(
    CardName::Slice,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 6,
            condition: DamageCondition::Always,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SLICE_PLUS: Entity = make_entity_card(
    CardName::Slice,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    0,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 9,
            condition: DamageCondition::Always,
        }, // +3 damage
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
