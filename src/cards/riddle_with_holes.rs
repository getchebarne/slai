use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 3,
        condition: DamageCondition::Always,
    },
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::CardTarget,
        selection: SelectionKind::Single,
    },
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 4,
        condition: DamageCondition::Always,
    }, // +1 damage
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::CardTarget,
        selection: SelectionKind::Single,
    },
};

pub static RIDDLE_WITH_HOLES: Entity = make_entity_card(
    CardName::RiddleWithHoles,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[HIT, HIT, HIT, HIT, HIT],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static RIDDLE_WITH_HOLES_PLUS: Entity = make_entity_card(
    CardName::RiddleWithHoles,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    true,
    &[HIT_PLUS, HIT_PLUS, HIT_PLUS, HIT_PLUS, HIT_PLUS],
    &[],
    &[],
    PlayRestriction::Always,
);
