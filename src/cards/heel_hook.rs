use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static HEEL_HOOK: Entity = make_entity_card(
    CardName::HeelHook,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 5,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::HeelHookProc,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static HEEL_HOOK_PLUS: Entity = make_entity_card(
    CardName::HeelHook,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 8,
                condition: DamageCondition::Always,
            }, // +3 damage
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::HeelHookProc,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
