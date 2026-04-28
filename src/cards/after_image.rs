use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static AFTER_IMAGE: Entity = make_entity_card(
    CardName::AfterImage,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    1,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::AfterImage,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    PlayRestriction::Always,
);
// Upgraded
pub static AFTER_IMAGE_PLUS: Entity = make_entity_card(
    CardName::AfterImage,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    1,
    true,
    false,
    true,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::AfterImage,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    PlayRestriction::Always,
);
