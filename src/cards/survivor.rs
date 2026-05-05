use crate::effect::{CandidatePool, DiscardSource, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SURVIVOR: Entity = make_entity_card(
    CardName::Survivor,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Basic,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SURVIVOR_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SURVIVOR.card_effects;
        a[0].kind = EffectKind::BlockGain { amount: 11 }; // +3 block
        a
    },
    ..SURVIVOR
};
