use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static DOPPELGANGER: Entity = make_entity_card(
    CardName::Doppelganger,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::XCost { offset: 0 },
    false,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DrawCardNextTurn,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static DOPPELGANGER_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost_kind: CardCostKind::XCost { offset: 1 }, // +1 offset
    ..DOPPELGANGER
};
