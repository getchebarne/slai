use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

const BOUNCE: Effect = Effect {
    kind: EffectKind::ModifierGain {
        kind: ModifierKind::Poison,
        stacks: 3,
    },
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::Monsters,
        selection: SelectionKind::Random { count: 1 },
    },
};

pub static BOUNCING_FLASK: Entity = make_entity_card(
    CardName::BouncingFlask,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[BOUNCE, BOUNCE, BOUNCE],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded: one more bounce
pub static BOUNCING_FLASK_PLUS: Entity = make_entity_card(
    CardName::BouncingFlask,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    false,
    &[BOUNCE, BOUNCE, BOUNCE, BOUNCE], // +1 bounce
    &[],
    &[],
    PlayRestriction::Always,
);
