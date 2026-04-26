use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static NOXIOUS_FUMES: Entity = make_entity_card(
    CardName::NoxiousFumes,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NoxiousFumes,
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
pub static NOXIOUS_FUMES_PLUS: Entity = make_entity_card(
    CardName::NoxiousFumes,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NoxiousFumes,
            stacks: 3, // +1
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
