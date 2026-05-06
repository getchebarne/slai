use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity, Tag};

pub static EVISCERATE: Entity = make_entity_card(
    CardName::Eviscerate,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    3,
    CardCostKind::MinusDiscardsThisTurn,
    false,
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
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 7 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 7 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
    &[Tag::Discard],
);
// Upgraded
pub static EVISCERATE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = EVISCERATE.card_effects;
        let upgraded_kind = EffectKind::DamagePhysical { amount: 9 }; // +2 damage
        a[0].kind = upgraded_kind;
        a[1].kind = upgraded_kind;
        a[2].kind = upgraded_kind;
        a
    },
    ..EVISCERATE
};
