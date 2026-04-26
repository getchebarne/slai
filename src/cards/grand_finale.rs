use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card_with_restriction};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static GRAND_FINALE: Entity = make_entity_card_with_restriction(
    CardName::GrandFinale,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    0,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 50 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
    PlayRestriction::DrawPileEmpty,
);
// Upgraded: +10 damage
pub static GRAND_FINALE_PLUS: Entity = make_entity_card_with_restriction(
    CardName::GrandFinale,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    0,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 60 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
    PlayRestriction::DrawPileEmpty,
);
