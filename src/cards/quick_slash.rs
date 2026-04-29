use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static QUICK_SLASH: Entity = make_entity_card(
    CardName::QuickSlash,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    PlayRestriction::Always,
);
// Upgraded
pub static QUICK_SLASH_PLUS: Entity = make_entity_card(
    CardName::QuickSlash,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    true,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 12 }, // +4 damage
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    PlayRestriction::Always,
);
