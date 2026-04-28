use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static FLECHETTES: Entity = make_entity_card(
    CardName::Flechettes,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::FlechettesDamage { damage: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
    PlayRestriction::Always,
);
// Upgraded
pub static FLECHETTES_PLUS: Entity = make_entity_card(
    CardName::Flechettes,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::FlechettesDamage { damage: 6 }, // +2 damage
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
    PlayRestriction::Always,
);
