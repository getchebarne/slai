use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical { amount: 4 },
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::Monsters,
        selection: SelectionKind::All,
    },
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical { amount: 6 }, // +2 damage
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
    false,
    false,
    false,
    false,
    &[HIT, HIT],
    PlayRestriction::Always,
);
// Upgraded
pub static DAGGER_SPRAY_PLUS: Entity = make_entity_card(
    CardName::DaggerSpray,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    true,
    false,
    false,
    false,
    &[HIT_PLUS, HIT_PLUS],
    PlayRestriction::Always,
);
