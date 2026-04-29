use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ENVENOM: Entity = make_entity_card(
    CardName::Envenom,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Envenom,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ENVENOM_PLUS: Entity = make_entity_card(
    CardName::Envenom,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    1, // -1 cost
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Envenom,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
