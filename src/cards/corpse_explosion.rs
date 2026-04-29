use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CORPSE_EXPLOSION: Entity = make_entity_card(
    CardName::CorpseExplosion,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 6,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::CorpseExplosion,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CORPSE_EXPLOSION_PLUS: Entity = make_entity_card(
    CardName::CorpseExplosion,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    true,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 9, // +3 poison
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::CorpseExplosion,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
