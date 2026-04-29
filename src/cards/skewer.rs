use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Skewer: X-cost attack, 7 (10+) damage per hit, target enemy. card_play
// multiplies card_effects by `energy.current + offset` (offset 0 here).
// At energy=3, fans out to 3 hits of 7 (or 10 upgraded).
pub static SKEWER: Entity = {
    let mut e = make_entity_card(
        CardName::Skewer,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        false,
        false,
        false,
        true,
        &[Effect {
            kind: EffectKind::DamagePhysical { amount: 7 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        }],
        PlayRestriction::Always,
    );
    e.card_cost_kind = CardCostKind::XCost { offset: 0 };
    e
};
// Upgraded: damage 7 -> 10 (per-hit); multiplier still X
pub static SKEWER_PLUS: Entity = {
    let mut e = make_entity_card(
        CardName::Skewer,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        true,
        false,
        false,
        true,
        &[Effect {
            kind: EffectKind::DamagePhysical { amount: 10 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        }],
        PlayRestriction::Always,
    );
    e.card_cost_kind = CardCostKind::XCost { offset: 0 };
    e
};
