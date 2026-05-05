use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static GRAND_FINALE: Entity = make_entity_card(
    CardName::GrandFinale,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 50,
            condition: DamageCondition::Always,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::DrawPileEmpty,
);
// Upgraded
pub static GRAND_FINALE_PLUS: Entity = make_entity_card(
    CardName::GrandFinale,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 60,
            condition: DamageCondition::Always,
        }, // +10 damage
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::DrawPileEmpty,
);
