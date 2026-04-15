use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static LEG_SWEEP: Entity = card_entity(
    CardName::LegSweep, CardKind::Skill, CardColor::Green, CardRarity::Uncommon,
    2, false, false, false, true,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 11 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded
pub static LEG_SWEEP_PLUS: Entity = card_entity(
    CardName::LegSweep, CardKind::Skill, CardColor::Green, CardRarity::Uncommon,
    2, true, false, false, true,
    &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 14, // +3 block
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 3, // +1 stack
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
);
