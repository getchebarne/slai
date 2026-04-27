use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Doppelganger: X-cost Skill, exhaust, target self. Next turn, draw +X
// cards and gain +X energy (+1 each if upgraded — handled via XCost offset).
// card_play multiplies the two ModifierGain card_effects by `energy.current
// + offset`. At energy=2 base: +2 stacks of each. Note: per StS,
// `if (effect > 0)` gates the action — multiplier 0 here means we emit
// zero copies (same observable behavior).
pub static DOPPELGANGER: Entity = {
    let mut e = make_entity_card(
        CardName::Doppelganger,
        CardKind::Skill,
        CardColor::Green,
        CardRarity::Rare,
        0,
        false,
        true, // exhaust
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
                    selection: SelectionKind::All,
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
                    selection: SelectionKind::All,
                },
            },
        ],
    );
    e.card_cost_kind = CardCostKind::XCost { offset: 0 };
    e
};
// Upgraded: offset 0 -> 1 (per StS `++effect`). card_effects unchanged.
pub static DOPPELGANGER_PLUS: Entity = {
    let mut e = make_entity_card(
        CardName::Doppelganger,
        CardKind::Skill,
        CardColor::Green,
        CardRarity::Rare,
        0,
        true,
        true, // exhaust
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
                    selection: SelectionKind::All,
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
                    selection: SelectionKind::All,
                },
            },
        ],
    );
    e.card_cost_kind = CardCostKind::XCost { offset: 1 };
    e
};
