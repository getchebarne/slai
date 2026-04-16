use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static FOOTWORK: Entity = card_entity(
    CardName::Footwork, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Dexterity,
            stacks: 2,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static FOOTWORK_PLUS: Entity = card_entity(
    CardName::Footwork, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, true, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Dexterity,
            stacks: 3, // +1 dexterity
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
