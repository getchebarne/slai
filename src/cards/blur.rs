use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BLUR: Entity = card_entity(
    CardName::Blur, CardKind::Skill, CardColor::Green, CardRarity::Uncommon,
    1, false, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 5 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Blur,
                stacks: 1,
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded
pub static BLUR_PLUS: Entity = card_entity(
    CardName::Blur, CardKind::Skill, CardColor::Green, CardRarity::Uncommon,
    1, true, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 8, // +3 block
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Blur,
                stacks: 1,
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
