use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 4,
        condition: DamageCondition::Always,
    },
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::Monsters,
        selection: SelectionKind::All,
    },
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 6,
        condition: DamageCondition::Always,
    }, // +2 damage
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::Monsters,
        selection: SelectionKind::All,
    },
};

pub static DAGGER_SPRAY: Entity = make_entity_card(
    CardName::DaggerSpray,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[HIT, HIT],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DAGGER_SPRAY_PLUS: Entity = make_entity_card(
    CardName::DaggerSpray,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    false,
    &[HIT_PLUS, HIT_PLUS],
    &[],
    &[],
    PlayRestriction::Always,
);
