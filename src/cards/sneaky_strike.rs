use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SNEAKY_STRIKE: Entity = make_entity_card(
    CardName::SneakyStrike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 12,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::SneakyStrikeProc { energy: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SNEAKY_STRIKE_PLUS: Entity = make_entity_card(
    CardName::SneakyStrike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    2,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 16,
                condition: DamageCondition::Always,
            }, // +4 damage
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::SneakyStrikeProc { energy: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
