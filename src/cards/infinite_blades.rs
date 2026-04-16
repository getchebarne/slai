use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static INFINITE_BLADES: Entity = card_entity(
    CardName::InfiniteBlades, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, false, false, false, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::InfiniteBlades,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static INFINITE_BLADES_PLUS: Entity = card_entity(
    CardName::InfiniteBlades, CardKind::Power, CardColor::Green, CardRarity::Uncommon,
    1, true, false, true, false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::InfiniteBlades,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
