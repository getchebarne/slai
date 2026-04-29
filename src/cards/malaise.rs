use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Malaise: X-cost Skill, exhaust, target enemy. Apply -X Strength and X Weak
// (X+1 if upgraded — handled via XCost offset). card_play multiplies the
// two card_effects by `energy.current + offset`.
pub static MALAISE: Entity = {
    let mut e = make_entity_card(
        CardName::Malaise,
        CardKind::Skill,
        CardColor::Green,
        CardRarity::Rare,
        0,
        false,
        true, // exhaust
        false,
        true,
        &[
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: -1,
                },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Weak,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
        ],
        PlayRestriction::Always,
    );
    e.card_cost_kind = CardCostKind::XCost { offset: 0 };
    e
};
// Upgraded: offset 0 -> 1 (per StS `++effect`). card_effects unchanged.
pub static MALAISE_PLUS: Entity = {
    let mut e = make_entity_card(
        CardName::Malaise,
        CardKind::Skill,
        CardColor::Green,
        CardRarity::Rare,
        0,
        true,
        true, // exhaust
        false,
        true,
        &[
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: -1,
                },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Weak,
                    stacks: 1,
                },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
        ],
        PlayRestriction::Always,
    );
    e.card_cost_kind = CardCostKind::XCost { offset: 1 };
    e
};
