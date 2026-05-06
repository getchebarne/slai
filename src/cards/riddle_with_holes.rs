use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical { amount: 3 },
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::CardTarget,
        selection: SelectionKind::Single,
    },
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical { amount: 4 }, // +1 damage
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
    &[],
);
// Upgraded
pub static RIDDLE_WITH_HOLES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = RIDDLE_WITH_HOLES.card_effects;
        a[0] = HIT_PLUS;
        a[1] = HIT_PLUS;
        a[2] = HIT_PLUS;
        a[3] = HIT_PLUS;
        a[4] = HIT_PLUS;
        a
    },
    ..RIDDLE_WITH_HOLES
};
