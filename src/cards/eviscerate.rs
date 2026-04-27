use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Eviscerate: 3-cost attack, 7 (9+) damage × 3, target enemy.
// Cost is reduced by 1 per card discarded this turn (per StS — handled by
// `CardCostKind::MinusDiscardsThisTurn` reading `state.cards_discarded_this_turn`).
pub static EVISCERATE: Entity = {
    let mut e = make_entity_card(
        CardName::Eviscerate,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        3,
        false,
        false,
        false,
        true,
        &[
            Effect {
                kind: EffectKind::DamagePhysical { amount: 7 },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
            Effect {
                kind: EffectKind::DamagePhysical { amount: 7 },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
            Effect {
                kind: EffectKind::DamagePhysical { amount: 7 },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
        ],
    );
    e.card_cost_kind = CardCostKind::MinusDiscardsThisTurn;
    e
};
// Upgraded: damage 7 -> 9 (per-hit), 3 hits unchanged
pub static EVISCERATE_PLUS: Entity = {
    let mut e = make_entity_card(
        CardName::Eviscerate,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        3,
        true,
        false,
        false,
        true,
        &[
            Effect {
                kind: EffectKind::DamagePhysical { amount: 9 },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
            Effect {
                kind: EffectKind::DamagePhysical { amount: 9 },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
            Effect {
                kind: EffectKind::DamagePhysical { amount: 9 },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::CardTarget,
                    selection: SelectionKind::All,
                },
            },
        ],
    );
    e.card_cost_kind = CardCostKind::MinusDiscardsThisTurn;
    e
};
