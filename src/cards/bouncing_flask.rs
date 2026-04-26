use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Bouncing Flask: amount of poison per bounce is fixed at 3 in StS; the
// magicNumber controls the number of bounces (3 base / 4 upgraded).
// Each bounce picks a random alive enemy independently — encoded as N
// separate Resolve { Monsters, Random{1} } effects so the resolver rolls
// fresh per call.
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
    false,
    false,
    false,
    false,
    &[BOUNCE, BOUNCE, BOUNCE],
);
// Upgraded: one more bounce
pub static BOUNCING_FLASK_PLUS: Entity = make_entity_card(
    CardName::BouncingFlask,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    true,
    false,
    false,
    false,
    &[BOUNCE, BOUNCE, BOUNCE, BOUNCE],
);
