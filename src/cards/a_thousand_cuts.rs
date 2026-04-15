use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static A_THOUSAND_CUTS: Entity = card_entity(
    CardName::AThousandCuts, CardKind::Power, CardColor::Green, CardRarity::Rare,
    2, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 1,
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static A_THOUSAND_CUTS_PLUS: Entity = card_entity(
    CardName::AThousandCuts, CardKind::Power, CardColor::Green, CardRarity::Rare,
    2, true, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 2, // +1 stack
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
