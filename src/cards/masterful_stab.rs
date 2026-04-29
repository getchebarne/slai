use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static MASTERFUL_STAB: Entity = make_entity_card(
    CardName::MasterfulStab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::GrowsOnDamageInstanceTaken,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 12 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static MASTERFUL_STAB_PLUS: Entity = make_entity_card(
    CardName::MasterfulStab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::GrowsOnDamageInstanceTaken,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 16 }, // +4 damage
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
