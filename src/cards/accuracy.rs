use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ACCURACY: Entity = card_entity(
    CardName::Accuracy, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Accuracy,
            stacks: 4,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static ACCURACY_PLUS: Entity = card_entity(
    CardName::Accuracy, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, true, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Accuracy,
            stacks: 6, // +2 stacks
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
