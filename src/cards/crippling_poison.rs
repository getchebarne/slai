use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CRIPPLING_POISON: Entity = make_entity_card(
    CardName::CripplingPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 4,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CRIPPLING_POISON_PLUS: Entity = make_entity_card(
    CardName::CripplingPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    true,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 7, // +3 poison
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Monsters,
                selection: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
