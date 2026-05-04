use crate::effect::{CandidatePool, DiscardSource, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ACROBATICS: Entity = make_entity_card(
    CardName::Acrobatics,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 3 },
            id_source: None,
            target: Target::Direct(None),
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
pub static ACROBATICS_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ACROBATICS.card_effects;
        a[0].kind = EffectKind::CardDraw { count: 4 }; // +1 draw
        a
    },
    ..ACROBATICS
};
