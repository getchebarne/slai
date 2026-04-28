use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SETUP: Entity = make_entity_card(
    CardName::Setup,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardSetupPick,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Hand,
            selection: SelectionKind::Input { count: 1 },
        },
    }],
    PlayRestriction::Always,
);
// Upgraded: cost 1 -> 0
pub static SETUP_PLUS: Entity = make_entity_card(
    CardName::Setup,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardSetupPick,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Hand,
            selection: SelectionKind::Input { count: 1 },
        },
    }],
    PlayRestriction::Always,
);
