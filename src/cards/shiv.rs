use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SHIV: Entity = make_entity_card(
    CardName::Shiv,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Special,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 4 },
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
pub static SHIV_PLUS: Entity = make_entity_card(
    CardName::Shiv,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Special,
    0,
    CardCostKind::Fixed,
    true,
    true,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 6 }, // +2 damage
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
